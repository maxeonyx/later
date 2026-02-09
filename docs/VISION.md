# Later: A Language for Graceful Cleanup

**later** is a programming language where cleanup is not an afterthought — it's intrinsic to how code composes.

## Core Principles

### 1. Linear Types by Default

Every value must be consumed exactly once. This isn't a restriction — it's a guarantee. When you acquire a resource, you *will* clean it up. The compiler ensures this.

```later
let file = open("data.txt")
# ... use file ...
file close   # must consume — compiler error if you forget
```

**Consumption** means destructuring into parts. Only the type's implementation knows how to split itself up. Each part is then recursively consumed.

**Auto-cleanup via `drop`**: Types can implement the `drop` symbol for automatic cleanup at scope end. This works when no extra information is needed (e.g., closing a file). Types that require a choice (e.g., commit vs rollback) deliberately don't implement `drop`, forcing explicit consumption.

```later
# File has drop — auto-closes at scope end
let file = open("data.txt")
file read
# file.drop() called automatically

# Transaction has NO drop — must choose
let tx = db.begin()
tx commit   # or tx rollback — compiler error if you forget
```

**Linearity hierarchy**: Not all types have the same ownership requirements:
- **Linear** (must consume exactly once, no drop): Transaction, unique tokens
- **Affine with drop** (consumed at most once, drop called if not consumed): File, Connection
- **Copyable** (can be used freely): Int, Float, Bool, String

The compiler tracks which category each type belongs to. Integers and booleans are freely copyable — you don't need to "consume" `42`.

### 2. Postfix Function Application

Functions are called in **postfix** style. The primary argument flows left-to-right via juxtaposition (no pipe character needed):

```later
5 double          # = double(5) = 10
5 add(3)          # = add(5, 3) = 8
urls map(get) all await   # left-to-right chain
```

**Functions have an implicit first argument** — the value flowing from the left. Explicit parameters are only for *additional* arguments:

```later
# Implicit first arg only
fn double { * 2 }
5 double          # = 10

# Implicit first arg + explicit extra params
fn add(b) { + b }
5 add(3)          # = 8

# Name the implicit arg when needed
fn process {
    as x
    if x > 10 { x * 2 } else { x + 1 }
}

# Anonymous blocks are functions too
5 { * 2 }         # = 10
paths map({ as path; "https://example.com/{path}" get })
```

**Pipeline arg = runtime data.** Explicit params can often be lifted to earlier stages (comptime/startup), leaving a function that just takes the runtime pipeline value.

**Postfix operators:**
- `.field` — field access
- `.[n]` — index access
- `?` — error propagation (send error effect if value is Err)

```later
get("api") await ? .[n] ? .thing ?
```

### 3. Symbols

Symbols are unique, opaque values used for:
- Effect names
- Special methods (like `drop`)
- Private object keys

```later
let my-key = symbol("my-key")  # debug name is optional
let obj = { [my-key]: "secret" }
obj[my-key]  # "secret"

# Two symbols with same debug name are still different
symbol("x") == symbol("x")  # false

# Symbols can't be converted to strings (opaque)
my-key to-string  # ERROR

# But debug() can show them (one-way door to I/O)
debug(my-key)  # prints: Symbol(my-key)
```

### 4. Effects (Koka-inspired)

Effects are declared capabilities that code can use. Handlers provide implementations.

```later
# Declare an effect
effect ask(prompt: String): Int

# Use it (called like any function)
fn get-sum {
    ask("first?") + ask("second?")
}

# Handle it — Koka-style `with`, scopes over rest of block
fn example() {
    with ask(prompt) {
        prompt print
        resume(42)
    }
    get-sum()
}
```

#### Handler Syntax

Handlers use `with effect(args) { body }`. The handler scopes over the **rest of the current block** — no wrapping braces around the handled code.

```later
fn example() {
    with error(e) {
        log("caught: {e}")
    }

    do-stuff() ?        # handled by the `with` above
    more-stuff() ?      # also handled
}
```

Handlers can be nested. Inner handlers shadow outer ones within their block:

```later
fn example() {
    with error(e) { log("outer: {e}") }

    do-stuff() ?                    # outer handler

    if condition {
        with error(e) { log("inner: {e}") }
        risky-thing() ?             # inner handler
    }

    other-stuff() ?                 # outer handler again
}
```

#### Resume Types

The effect declaration specifies how `resume` can be used. The handler syntax is **symmetric** across all four cases — only `resume` availability differs:

| Return type | `resume` in handler | Meaning |
|-----------|---------------------|---------|
| `: Never` | Not available | Abort/unwind |
| `: T` | Must call once | Exactly once (default) |
| `: T where resume: Many` | Call zero or more | Sequential generator |
| `: T where resume: Fork` | Call concurrently | Parallel exploration |

```later
# Never — no resume (abort)
effect fail(msg: String): Never

fn example() {
    with fail(msg) {
        print("caught: {msg}")
        # no resume available — handler returns directly
    }
    work()
}

# Once — resume exactly once (default)
effect ask(prompt: String): Int

fn example() {
    with ask(prompt) {
        prompt print
        resume(read-int())
    }
    ask("number?") + ask("another?")
}

# Many — resume zero or more times sequentially (generator)
effect yield(value: Int): ()

fn example() {
    with yield(v) {
        print("got: {v}")
        resume(())
    }
    generate-values()
}

# Fork — resume concurrently (values must be copyable)
effect choose(): Bool

fn example() {
    with choose() {
        resume(true)
        resume(false)
    }
    explore-choices()
}
```

#### Built-in Effects

- **`error`** — a `Never` effect for recoverable errors (used with `?`)
- **`cancel`** — a `Never` effect for cancellation
- **`alloc`** — heap memory allocation (see section 9)

### 5. Cancellation & Structured Concurrency

Cancellation is a `Never` effect that propagates through the task tree.

**Effect visibility:**
- Public effects (symbol exposed) can be handled
- Private effects (symbol hidden) must propagate

**Tree unwinding:**

```
A
└─ B
   ├─ C1
   └─ C2
      └─ D1 ← Never effect here
```

1. D1's `Never` propagates up to C2
2. C2 exits, propagates to B
3. B has two children — must wait for both
4. B triggers cancellation in C1 (at its deepest point)
5. B waits for C1 to exit
6. Only then does B propagate up to A

**Never leave orphans**: A parent cannot exit until all children have exited. This is true structured concurrency.

**ExceptionGroup**: When multiple `Never` effects propagate up, they form a structure representing the abridged stack tree.

#### DAG Stacks

Tasks can form a DAG, not just a tree. One subtask can have multiple parents. When a DAG-shared subtask cancels, **all parents are notified**.

### 6. Operator Precedence: None

**BODMAS is buried.** 🪦

All operators evaluate left-to-right. Use `()` to group.

```later
2 + 3 * 4      # = (2 + 3) * 4 = 20, not 14
true or false and false  # = (true or false) and false = false
```

**Rationale**: "Write forward, evaluate forward." No mental backtracking to figure out what binds to what. You (or an LLM) can write code incrementally without needing to go back and add parentheses.

### 7. Comments & Documentation

```later
#! shebang (allowed because # is a comment)

# normal comment (ignored in docs)

#*
block comment (ignored in docs)
*#

#*
#*
nested block comments work
*#
*#

## doc comment (two ##)
### sub-heading
#### heading
##### page title

##*
Multi-line doc comment.

Everything in here is documentation.
Code blocks run AND appear in docs (literate programming):
```
print("hello")
```
*##
```

**Reverse of Markdown**: More `#` = deeper nesting. This lets doc comments naturally nest within code structure:

```later
{
    ### Module Name (heading)
    
    ## This module does X and Y.
    
    fibonacci: fn() {
        ### Fibonacci
        ## Returns first two Fibonacci numbers.
        ## This becomes a sub-heading of the parent object!
        [0, 1]
    },
}
```

**Heading levels follow object structure**: Nesting of headings happens through objects/modules, not functions. A function's doc is a sub-heading of its containing object.

**Literate programming**: Code blocks in doc comments (without language marker) are both executed and included in generated docs.

**Runtime access**:
```later
# help() returns documentation of any value
let text = help(my-fn)
let md = help(my-fn, "markdown")
let html = help(my-fn, "html")
```

### 8. Fallible Cleanup

Cleanup can fail. This is reality — disks get unplugged, networks go down. The type system acknowledges this.

When an error occurs during cleanup:
- The first error wins (becomes the propagated error)
- Cleanup errors are logged
- All cleanup still runs

### 9. Memory Allocation as an Effect

Memory allocation is an effect. Code that doesn't allocate doesn't have the `alloc` effect — the type system tracks this.

```later
# Pure computation — no alloc effect
fn add(b) { + b }

# Stack allocation of known-size values — no alloc effect needed
fn make-point(x, y) { { x, y } }

# Heap allocation — requires alloc effect
fn make-list(items) {
    items to-list  # needs alloc
}
```

#### Size Taxonomy

Four categories, forming a 2×2 matrix:

|                  | Known size          | Unknown size         |
|------------------|--------------------|-----------------------|
| **Static alloc** | Stack / inline     | Bounded (MaxSize)     |
| **Dynamic alloc**| Fixed heap alloc   | Growable (Vec-like)   |

- **Known + Static**: `[u8; 64]` — size known at compile time, stack allocated
- **Known + Dynamic**: Size known but too large for stack — heap allocated, fixed
- **Unknown + Bounded**: `List(MaxSize(100))` — max size known, can pre-allocate
- **Unknown + Dynamic**: Growable containers — requires `alloc` effect

Stack allocation of known-size values doesn't require the alloc effect. Only heap/dynamic allocation does. The compiler needs to know sizes at compile time to determine whether alloc is needed.

#### Interaction with Stages

- **Comptime**: No alloc (or special comptime allocator)
- **Startup**: Alloc allowed. Sizes may come from config. Memory can be pre-allocated.
- **Runtime**: Full alloc. This is where the `alloc` effect matters most.

### 10. Composable Cleanup

Cleanup behavior emerges from how primitives compose:
- **Scope**: multiple resources clean up in reverse acquisition order
- **Struct**: struct cleanup composes field cleanups
- **Collection**: collection cleanup cleans up all elements
- **Task**: task cleanup includes all owned resources

### 11. Upward-Propagating Memory Information

Types can carry information about their memory footprint. This propagates upward through composition, enabling:
- Compile-time memory allocation when sizes are static
- Startup-time allocation when sizes depend on config
- Runtime allocation as a fallback

### 12. Multistage Programming

Building is running. The program executes in stages:

1. **Build time**: produces a residual program
2. **Startup time**: ingests config, specializes further
3. **Runtime**: actual execution

Like Zig's comptime, but with arbitrary stages.

### 13. Blocks and Objects

**`{}` is always an object literal.** Even empty `{}` is an empty object.

**Blocks** appear after keywords (`if`, `fn`, `loop`, `defer`, `handle`, `spawn`, `nursery`, `@comptime`, `@startup`). These keyword-introduced blocks use `{}` — the keyword disambiguates.

**Standalone multi-statement expressions** use `()`:
```later
let x = (
    let a = 1
    let b = 2
    a + b
)
```

**Smart disambiguation (preferred over `()`)**: When `{` appears in expression position without a keyword, the parser can often determine from the first tokens whether it's an object or not:
- `{ key:` → object
- `{ ...expr` → object spread
- `{ [expr]:` → computed key object
- `{ identifier,` → object shorthand

Note: `{ x }` is ambiguous (object shorthand `{x: x}` vs single-expression block). Object keys may be arbitrary expressions (`key_expr: value_expr`), which adds complexity. Fall back to `()` for multi-statement expressions if disambiguation fails.

## Syntax Summary

```later
# Comments
# this is a comment
## this is a doc comment

# Let bindings
let x = 42
let mut y = 0

# Functions (implicit first arg)
fn double { * 2 }
fn add(b) { + b }
fn process { as x; x + 1 }

# Postfix application (left-to-right)
5 double              # = 10
5 add(3)              # = 8
x f g(y) h            # = h(g(f(x), y))

# Postfix operators
obj.field             # field access
list.[n]              # index access
value ?               # error propagation

# Objects and lists
{ a: 1, b: 2 }
[1, 2, 3]

# Effects — declare and handle
effect ask(prompt: String): Int

with ask(prompt) {
    prompt print
    resume(read-int())
}
ask("number?")

# Defer — single-path cleanup
defer { resource close await }

# Spawn and structured concurrency
nursery {
    spawn { work-a() }
    spawn { work-b() }
}
```

## Showcase Examples

These examples demonstrate how multiple features interact cohesively.

### Async Server with Cleanup

Linear types + defer + structured concurrency + await in cleanup:

```later
fn serve(addr) {
    let sock = listen(addr)
    defer { sock shutdown await }

    nursery {
        loop {
            let conn = sock accept await
            spawn {
                defer { conn close await }
                conn handle-request await
            }
        }
    }
}
```

- `sock` is affine+drop — `defer` provides the drop path
- `defer` runs on any exit: normal, error, or cancellation
- `await` inside `defer` works because cancellation is cooperative
- `nursery` scopes all spawned tasks — when the loop exits, nursery waits for all children

### Transaction with Must-Move Semantics

Linear types + effect handlers + split cleanup paths:

```later
fn transfer(db, from, to, amount) {
    let tx = db begin-transaction

    with error(e) {
        tx rollback await
        throw(e)
    }

    tx execute("UPDATE accounts SET balance = balance - {amount} WHERE id = {from}") await ?
    tx execute("UPDATE accounts SET balance = balance + {amount} WHERE id = {to}") await ?
    tx commit await
}
```

- `tx` is linear (no drop) — must be consumed exactly once
- `with error(e)` handles the error path → rollback, then re-throw
- If no error, happy path reaches `commit`
- Compiler verifies `tx` is consumed on ALL exit paths

### Generator via Effect

Effects + resume + postfix application:

```later
effect yield(value: Int): ()

fn fibonacci() {
    let mut a = 0
    let mut b = 1
    loop {
        yield(a)
        let next = a + b
        a = b
        b = next
    }
}

fn example() {
    let mut count = 0
    with yield(v) {
        print("fib: {v}")
        count = count + 1
        if count < 10 { resume(()) }
        # no resume = generator stops
    }
    fibonacci()
}
```

## Target Platforms

- Native (primary)
- WASM (first-class support — cancellation via flag checking works here)

## Ancestry

`later` inherits from two earlier language experiments:
- **kal** — effects for IO/generators/errors, symbols, `#` comment system, `send`/`handle`, multistage
- **raro** — postfix function application, left-to-right precedence, `as` bindings, kebab-case, implicit first argument

Reference copies of both are in `kal.ignore/` and `raro.ignore/`.
