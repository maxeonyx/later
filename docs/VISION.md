# Later: A Language for Graceful Cleanup

**later** is a programming language where cleanup is not an afterthought - it's intrinsic to how code composes.

## Core Principles

### 1. Linear Types by Default

Every value must be consumed exactly once. This isn't a restriction - it's a guarantee. When you acquire a resource, you *will* clean it up. The compiler ensures this.

```later
let file = open("data.txt")
# ... use file ...
file | close   # must consume - compiler error if you forget
```

**Consumption** means destructuring into parts. Only the type's implementation knows how to split itself up. Each part is then recursively consumed.

**Auto-cleanup via `drop`**: Types can implement the `drop` symbol for automatic cleanup at scope end. This works when no extra information is needed (e.g., closing a file). Types that require a choice (e.g., commit vs rollback) deliberately don't implement `drop`, forcing explicit consumption.

```later
# File has drop - auto-closes at scope end
let file = open("data.txt")
file | read
# file.drop() called automatically

# Transaction has NO drop - must choose
let tx = db.begin()
tx | commit   # or tx | rollback - compiler error if you forget
```

### 2. Symbols

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
my-key | to-string  # ERROR

# But debug() can show them (one-way door to I/O)
debug(my-key)  # prints: Symbol(my-key)
```

### 3. Effects (Koka-inspired)

Effects are declared capabilities that code can use. Handlers provide implementations.

```later
# Declare an effect
effect ask(prompt: String): resume(Int)

# Use it (looks like a function call)
fn get-sum() {
    ask("first?") + ask("second?")
}

# Handle it
handle ask(prompt) {
    print(prompt)
    42  # implicit resume(42) for FnOnce
}
get-sum()
```

#### Resume Types

The effect declaration specifies how `resume` can be used:

| Signature | `resume` in handler | Meaning |
|-----------|---------------------|---------|
| `: Never` | Not available | Abort/unwind |
| `: resume(T)` | `FnOnce(T)` | Exactly once (implicit via return) |
| `: resume(T) where resume: FnMut` | `FnMut(T)` | Zero or more, sequential |
| `: resume(T) where resume: Fn` | `Fn(T)` | Zero or more, concurrent |

```later
# Abort - no resume, triggers unwinding
effect fail(msg: String): Never

handle fail(msg) {
    print("caught: {msg}")
    # no resume available
}

# Generator - explicit resume, multiple sequential calls
effect yield(value: Int): resume(()) where resume: FnMut

handle yield(v) {
    print("got: {v}")
    resume(())  # explicit for FnMut
}

# Fork - resume can be called concurrently (values must be Clone)
effect choose(): resume(Bool) where resume: Fn

handle choose() {
    resume(true)
    resume(false)
}
```

#### FnMut vs Fn (Clone)

- `FnMut`: Multiple sequential resumes OK. Values at effect site can be mutable.
- `Fn` (Clone): Multiple concurrent resumes. All captured values must be Clone. Creates parallel "timelines".

### 4. Cancellation & Structured Concurrency

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
3. B has two children - must wait for both
4. B triggers cancellation in C1 (at its deepest point)
5. B waits for C1 to exit
6. Only then does B propagate up to A

**Never leave orphans**: A parent cannot exit until all children have exited. This is true structured concurrency.

**ExceptionGroup**: When multiple `Never` effects propagate up, they form a structure representing the abridged stack tree.

#### DAG Stacks

Tasks can form a DAG, not just a tree. One subtask can have multiple parents:

```later
fn start-task1(subtask) { subtask | await + 1 }
fn start-task2(subtask) { subtask | await + 3 }

fn start-parent(dest) {
    get-thingy-from(dest)  # returns a subtask
    | split {
        > start-task1 as t1
        > start-task2 as t2
    }
    | await all
}
```

When a DAG-shared subtask cancels, **all parents are notified**.

### 5. Operator Precedence: None

**BODMAS is buried.** 🪦

All operators evaluate left-to-right. Use `()` or `{}` to group.

```later
2 + 3 * 4      # = (2 + 3) * 4 = 20, not 14
true or false and false  # = (true or false) and false = false
```

**Rationale**: "Write forward, evaluate forward." No mental backtracking to figure out what binds to what. You (or an LLM) can write code incrementally without needing to go back and add parentheses.

### 6. Comments

```later
# Line comment
## Doc comment
```

### 7. Fallible Cleanup

Cleanup can fail. This is reality - disks get unplugged, networks go down. The type system acknowledges this.

When an error occurs during cleanup:
- The first error wins (becomes the propagated error)
- Cleanup errors are logged
- All cleanup still runs

### 8. Composable Cleanup

Cleanup behavior emerges from how primitives compose:
- **Scope**: multiple resources clean up in reverse acquisition order
- **Struct**: struct cleanup composes field cleanups
- **Collection**: collection cleanup cleans up all elements
- **Task**: task cleanup includes all owned resources

### 9. Upward-Propagating Memory Information

Types can carry information about their memory footprint. This propagates upward through composition, enabling:
- Compile-time memory allocation when sizes are static
- Startup-time allocation when sizes depend on config
- Runtime allocation as a fallback

### 10. Multistage Programming

Building is running. The program executes in stages:

1. **Build time**: produces a residual program
2. **Startup time**: ingests config, specializes further
3. **Runtime**: actual execution

Like Zig's comptime, but with arbitrary stages.

## Syntax Summary

```later
# Comments
# this is a comment
## this is a doc comment

# Let bindings
let x = 42
let mut y = 0

# Functions
fn add(a, b) { a + b }
fn double(x) x * 2     # single-expression

# Pipe (left-to-right)
x | f | g              # = g(f(x))

# Objects and lists
{ a: 1, b: 2 }
[1, 2, 3]

# Effects
effect ask(): resume(Int)
handle ask() { 42 }

# Defer
defer { cleanup-code }

# Spawn
spawn { work } as task
task | await
```

## Target Platforms

- Native (primary)
- WASM (first-class support - cancellation via flag checking works here)
