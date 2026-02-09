# Design Reconciliation Review

**Date**: 2026-02-08, updated 2026-02-09
**Status**: Walkthrough with Max in progress. VISION.md and PLAN.md updated. Tests not yet rewritten.

This document captures contradictions, gaps, and design decisions found by reviewing all 332 example files, the test suite, and VISION.md together.

## Resolution Status

| # | Issue | Status |
|---|-------|--------|
| 1a | Comment syntax `#` vs `//` | ✅ RESOLVED — `#` is correct, PLAN.md was wrong |
| 1b | Effect declaration syntax | ✅ RESOLVED — simple `: T` style, not `resume(T)` |
| 1c | Handler syntax (6 forms) | ⏳ OPEN — needs consolidation, not yet discussed |
| 1d | Effect invocation `send` vs fn-call | ✅ RESOLVED — function-call style |
| 2a | Effect declaration vs symbol | ⏳ OPEN — not yet discussed |
| 2b | `panic` as built-in effect | ✅ RESOLVED — yes, built-in Never effect |
| 2c | Compile-time effect tracking | ⏳ OPEN — confirms full effect type system needed |
| 3a | What makes a type linear | ✅ RESOLVED — linearity hierarchy (linear/affine+drop/copyable) |
| 3b | Linear struct field access | ⏳ OPEN — dot borrows, destructuring moves (proposed) |
| 3c | Linear list indexing | ⏳ OPEN — needs special iteration/destructuring |
| 4 | Allocation as effect | ✅ RESOLVED — `alloc` effect, size taxonomy |
| 5 | `{}` ambiguity | ✅ RESOLVED — `{}` always object, `()` for standalone blocks |
| 6 | Comment syntax | ✅ RESOLVED — same as 1a |
| NEW | Postfix application | ✅ RESOLVED — juxtaposition replaces `|` pipe |
| NEW | Implicit first argument | ✅ RESOLVED — raro-style, explicit params for extras only |
| NEW | Pipeline = runtime data | ✅ RESOLVED — explicit params liftable to earlier stages |

## Table of Contents

1. [Syntax Contradictions](#1-syntax-contradictions)
2. [Effect System Inconsistencies](#2-effect-system-inconsistencies)
3. [Linear Types Gaps](#3-linear-types-gaps)
4. [Memory & Allocation (NEW)](#4-memory--allocation-new)
5. [Ambiguity: Empty Object vs Empty Block](#5-ambiguity-empty-object-vs-empty-block)
6. [Comment Syntax Contradiction](#6-comment-syntax-contradiction)
7. [Open Design Questions](#7-open-design-questions)
8. [Implementation Feasibility Notes](#8-implementation-feasibility-notes)

---

## 1. Syntax Contradictions

### 1a. Comment syntax: `#` vs `//`

**VISION.md** uses `#` for comments (sections 2, 6). The comment system is deeply designed around `#`:
- `#` = line comment
- `#*...*#` = block comment
- `##` = doc comment
- `###` = doc sub-heading
- `#!` = shebang

**PLAN.md** (decision #6) says: "Comments: `//` for line comments (like Rust/JS, not `#` like Raro)"

**All 332 example files** use `#` for comments.

**Resolution needed**: The examples and VISION.md are consistent with `#`. PLAN.md is wrong. The `#`-based system is clearly the intended design — it's deeply thought out with doc comments, block comments, and heading levels all using `#`. This should be confirmed and PLAN.md updated.

### 1b. Effect declaration syntax

**VISION.md** uses Koka-inspired declaration syntax:
```later
effect ask(prompt: String): resume(Int)
effect fail(msg: String): Never
effect yield(value: Int): resume(()) where resume: FnMut
```

**Example files** use a simpler syntax:
```later
effect ask(): Int          # effect_continue.later
effect greet(): String     # effect_simple.later
effect ask(): String       # effect_nested.later
effect yield(value: Int): ()  # effect_compose.later
effect fail(msg: String): Never  # effect_finally.later
```

The examples don't use `resume(T)` syntax — they just use the return type directly. The `resume` concept appears only in handler bodies.

**Resolution needed**: The example syntax is cleaner. The VISION.md Koka-style `resume(T)` declaration is verbose. I'd recommend the example style as canonical:
- `: T` means resume once with T (FnOnce, the default)
- `: Never` means no resume (abort)
- `: T where resume: FnMut` for generators
- `: T where resume: Fn` for forking

### 1c. Handler syntax

**VISION.md** shows:
```later
handle ask(prompt) {
    print(prompt)
    42  # implicit resume
}
get-sum()
```

**Examples** show multiple different handler styles:
```later
# Style 1: handle before call (block-less)
handle greet() { "Hello, World!" }
say-hello()

# Style 2: handle wrapping a block
handle {
    tasks | all | await
} error e {
    log = ["error: " + e, ...log]
}

# Style 3: resume as explicit parameter
handle ask(resume) { resume(50) }
handle ask(q, resume) { resume("World") }

# Style 4: resume with Drop annotation
handle fail(msg, resume: Drop) { ... }

# Style 5: shallow modifier
handle shallow emit(v) { print("inner: {v}") }

# Style 6: pipe-style handle
work() | handle { panic msg { print("error: {msg}") } }
```

These are 6 different syntactic forms. They need to be consolidated into one coherent grammar.

**Resolution needed**: Define one canonical handler syntax. My analysis of what's actually needed:
- A handler needs: effect name, parameter bindings, resume access, body
- Handlers can be "wrapping" (scope a block) or "ambient" (active for rest of scope)
- `shallow` vs `deep` modifier
- `resume: Drop` to opt out of resuming (for Never effects where you want to inspect but not resume)

### 1d. Effect invocation: `send ... with` vs function-call style

**VISION.md** doesn't show `send ... with` — it shows effects called like functions:
```later
ask("first?") + ask("second?")
```

**Some examples** use `send ... with`:
```later
send yield with i       # generator.later
send error with "task 2 failed"  # all_fail.later
send io-error with "transient failure"  # cleanup_retry.later
```

**Other examples** use function-call style:
```later
ask()          # effect_continue.later
greet()        # effect_simple.later
emit(1)        # effect_deep.later
log("msg")     # effect_compose.later
fail("boom")   # effect_finally.later
```

**Resolution needed**: Pick one. The function-call style is used far more often and reads better. `send X with Y` feels like a lower-level primitive. Recommendation: effects are invoked like functions. `send` could be reserved for dynamic/computed effect dispatch if ever needed.

## 2. Effect System Inconsistencies

### 2a. Effect declaration vs. effect-as-symbol

**VISION.md** (section 2) says symbols are used for effect names. This implies effects are dynamically dispatched via symbol identity.

**Examples** use `effect foo(): T` declarations which look statically declared.

But some examples use symbols more dynamically:
```later
let error = symbol()   # defer_on_error.later — creates a symbol, uses it as effect name
```

**Resolution needed**: Are effects:
(a) Statically declared types (like Koka) — `effect ask(): Int`
(b) Dynamic symbols that can be sent — `let my-effect = symbol(); send my-effect with value`
(c) Both — static declarations create symbols, dynamic use also possible

Option (c) is most powerful but hardest to type-check. Option (a) is simplest to implement and reason about. The examples mostly use (a) with a few hints of (b).

### 2b. `panic` as effect

Several examples use `panic(msg)` like a built-in effect:
```later
panic("oops")                    # linear_panic_cleanup.later
panic("original error")          # error_in_cleanup.later
```

And handle it like an effect:
```later
work() | handle { panic msg { print("error: {msg}") } }
```

But `panic` is never declared with `effect`. Is it a built-in Never effect? This should be explicit in VISION.md.

### 2c. Unhandled effects: compile-time vs runtime error?

`effect_unhandled.later` says "compile error" — meaning the compiler tracks effect types.
But `effect_handler_scope.later` also says "compile error" for scope escapes.

This implies a full effect type system (like Koka). This is a MAJOR implementation complexity. The compiler must track which effects each function can perform and verify handlers are in scope.

**Resolution needed**: Confirm: Later has a compile-time effect type system. This is a big deal for implementation.

## 3. Linear Types Gaps

### 3a. What makes a type "linear"?

The examples assume `open()` returns a linear type. But there's no syntax shown for *declaring* a type as linear. Questions:

- Are ALL types linear by default (as VISION.md section 1 says)?
- If so, how do "normal" values like integers work? `let x = 42` — must you consume `x`?
- How do you opt out of linearity? (VISION.md mentions `drop` for auto-cleanup, but what about truly non-linear types like Int?)

**Resolution needed**: Clarify the linearity hierarchy:
- **Linear** (must consume exactly once, no drop): Transaction, unique tokens
- **Affine with drop** (consumed at most once, drop called if not consumed): File, Connection
- **Copyable/non-linear** (can be used freely): Int, String, Bool

Most practical languages need all three. VISION.md says "every value consumed exactly once" but Int clearly isn't linear in practice. The examples freely use integers multiple times (`i + 1`, `count + 1`, etc.).

### 3b. `linear_struct.later` vs `linear_split.later` contradiction

`linear_struct.later` accesses fields via dot notation AND consumes them:
```later
resources.a | close
resources.b | close
```

`linear_field_move.later` says this is an ERROR:
```later
let f = record.file  # Error: can't move field out
```

But the first example does exactly that via pipe! `resources.a | close` moves `resources.a` into `close`. These contradict.

**Resolution needed**: Either:
(a) Dot access on linear struct moves the field (and the struct becomes partially consumed) — then `linear_struct.later` is correct
(b) Dot access borrows — then `linear_struct.later` needs destructuring syntax

`linear_split.later` shows destructuring: `let { a, b } = pair`. This is the clean approach. Field access should borrow; destructuring should move.

### 3c. `linear_list.later` — indexing into linear lists

```later
files[0] | close
files[1] | close
files[2] | close
```

This has the same problem. `files[0]` moves an element out of a list. But `linear_list_move.later` says this is an error. Contradiction.

**Resolution needed**: Lists of linear values probably need special iteration/destructuring rather than random index access. E.g.:
```later
let [a, b, c] = files
a | close
b | close
c | close
```

## 4. Memory & Allocation (NEW)

Max wants:
- Memory allocation as an effect (no alloc = code without that effect)
- Distinguish static vs dynamic allocation
- Distinguish known-size vs unknown-size

### 4a. Allocation as an effect

This is brilliant and fits naturally with the effect system. Proposal:

```later
# Built-in effect
effect alloc(size: Size): resume(Ptr) where resume: FnOnce

# Code that doesn't use alloc has no alloc effect
# The type system tracks this!
fn add(a: Int, b: Int): Int {  # no alloc effect
    a + b
}

fn make-list(n: Int): List(Int) {  # has alloc effect
    let mut result = []   # alloc!
    # ...
}
```

A handler can provide different allocators:
```later
# Arena allocator
handle alloc(size) {
    arena | bump-alloc(size)
}

# No-alloc context (compile error if alloc is used)
# Just... don't provide a handler. The effect is unhandled = compile error.
```

### 4b. Size taxonomy

Four categories, forming a 2×2 matrix:

|                  | Known size          | Unknown size         |
|------------------|--------------------|-----------------------|
| **Static alloc** | Stack / inline     | Bounded (MaxSize)     |
| **Dynamic alloc**| Fixed heap alloc   | Growable (Vec-like)   |

- **Known + Static**: `[u8; 64]` — size known at compile time, no heap allocation needed
- **Known + Dynamic**: Size known but too large for stack — heap allocated, fixed
- **Unknown + Static (Bounded)**: `List(MaxSize(100))` — max size known, can pre-allocate
- **Unknown + Dynamic**: Growable containers — need alloc effect

### 4c. How this interacts with stages

- **Comptime**: No alloc (or special comptime allocator). Already enforced — `comptime_io.later` bans IO.
- **Startup**: Alloc allowed. Size might come from config. Memory can be pre-allocated.
- **Runtime**: Full alloc. This is where the alloc effect matters most.

### 4d. Effect levels for allocation

```later
# No allocation — pure computation
fn add(a: Int, b: Int): Int { a + b }

# Stack allocation only — known size, no effect needed
fn make-point(x: Float, y: Float): { x: Float, y: Float } {
    { x, y }  # struct fits on stack, no alloc effect
}

# Heap allocation — requires alloc effect
fn make-list(items: ...Int): List(Int) {
    items | to-list  # needs alloc
}
```

**Key insight**: Stack allocation of known-size values shouldn't require the alloc effect. Only heap/dynamic allocation should. This means the compiler needs to know sizes at compile time to determine whether alloc is needed.

### 4e. Open questions for allocation

- Does string concatenation require alloc? (Probably yes — strings are variable-size)
- Does list literal `[1, 2, 3]` require alloc? (Known size — could be stack. But what about `[...a, ...b]`?)
- How does this interact with closures? (Closures capture environment — may need alloc)
- Should there be a `stack-alloc` vs `heap-alloc` distinction in effects?

## 5. Ambiguity: Empty Object vs Empty Block

`empty_object.later` expects `{}` to print `{}` (empty object).
`block_empty.later` expects `{}` to print `nil` (empty block).

These are THE SAME SYNTAX with different expected outputs. This is a real ambiguity that must be resolved.

**Resolution options**:
(a) Context-dependent: `let x = {}` is object, bare `{}` is block
(b) Empty object requires something: `{:}` or `Object()` or `{ }` (with space?)
(c) `{}` is always a block, empty object is `Object()` or `{,}`
(d) `{}` is always an empty object, empty block is `do {}` or `begin...end`

Recommendation: (a) with a simple rule — `{}` is a block unless it contains `:` or is in expression position after `=`. This is what JavaScript does and people are used to it.

But actually, the test expects BOTH to work differently. The test for `empty_object.later` has `{}` as the only expression, and `block_empty.later` also has `{}` as the only expression. They literally have the same code but expect different output. **One of these tests is wrong.**

## 6. Comment Syntax Contradiction

Already covered in 1a, but to emphasize: PLAN.md decision #6 says `//` but everything else says `#`. PLAN.md is wrong.

## 7. Open Design Questions (Consolidated)

### Already resolved in examples (should be added to VISION):
1. ✅ Comments use `#` (not `//`)
2. ✅ Booleans: `true`, `false`, `and`, `or`, `not`
3. ✅ Cleanup: `defer { }` blocks
4. ✅ Pipe: `x | f` calls `f(x)`
5. ✅ Precedence: left-to-right, no BODMAS
6. ✅ `nil` exists as unit/nothing value
7. ✅ String interpolation: `"value is {x}"`
8. ✅ Negative indexing: `list[-1]`
9. ✅ `while` loop as sugar for `loop { if !cond { break } }`
10. ✅ `match` expression exists
11. ✅ Pattern guards: `n if n > 0 { ... }`
12. ✅ Or patterns: `1 or 2 or 3 { ... }`
13. ✅ `nursery { }` for structured concurrency scope
14. ✅ `channel()` for inter-task communication
15. ✅ `assert()` built-in
16. ✅ `debug()` built-in
17. ✅ `typeof()` built-in
18. ✅ Type annotations: `let x: Int = 42`, `fn(x: Int): Int`
19. ✅ Type definitions: `type Node = { value: Int, next: Node? }`
20. ✅ Generics: `fn identity<T>(x: T) -> T`
21. ✅ Named arguments: `f(x: 1, y: 2)`
22. ✅ Default parameters: `fn f(x = 42)`
23. ✅ Rest parameters: `fn f(a, ...rest)`
24. ✅ `import` / `export` (last expression is export)

### Still open:
1. **Empty object vs empty block** — `{}` ambiguity (see section 5)
2. **Effect invocation** — `send X with Y` vs function-call `X(args)` (see section 1d)
3. **Effect declaration** — verbose Koka style vs simple return-type style (see section 1b)
4. **Handler syntax** — too many forms, needs consolidation (see section 1c)
5. **Linearity default** — what types are actually linear? (see section 3a)
6. **Field access on linear structs** — borrow or move? (see section 3b)
7. **Defer capture** — by value or by reference? (examples show `defer { print(x) }` where x is `let mut x = 1`, expects to print 1 not 2 — so by-value capture at defer time)
8. **Pipe with arguments** — `x | f(y)` means `f(x, y)` (partial application)? Examples confirm yes.
9. **Method-style pipe** — `"hello" | .len()` — is this real or sugar?
10. **Chained comparisons** — `1 < 2 < 3` expands to `(1 < 2) and (2 < 3)` — is this special syntax?
11. **Allocation as effect** — NEW (see section 4)
12. **Static vs dynamic memory** — NEW (see section 4)

## 8. Implementation Feasibility Notes

### High risk / high effort:
- **Full effect type system** — tracking which effects each function performs. This is research-level PL work (Koka, Eff, Frank). Consider whether effects should be inferred or declared.
- **Multi-resume / forking effects** — `resume: Fn` (Clone) that creates parallel timelines is extremely complex. It's essentially delimited continuations with copying.
- **DAG-structured concurrency** — tasks with multiple parents. Much harder than tree-structured.
- **Multistage evaluation** — comptime/startup/runtime staging with residual programs. This is partial evaluation, also research-level.

### Medium effort:
- **Linear type checker** — tracking ownership, consumption, borrowing
- **Cancellation** — flag checking, structured unwinding
- **Allocation effect** — needs size inference to determine what requires alloc
- **Tail call optimization** — `fn_tail_rec.later` expects 1M recursion depth

### Suggested simplifications for v1:
1. Start with **inferred** effect types (like Koka's inference), not declared
2. Skip multi-resume / forking effects initially
3. Tree-structured concurrency only (no DAG)
4. Two stages only: comptime + runtime (skip startup)
5. Allocation effect can be v2 — just track sizes for now

---

## Action Items

After Max reviews this document:

1. **Resolve the contradictions** — especially `{}`, comment syntax, effect syntax
2. **Update VISION.md** with resolved decisions
3. **Fix conflicting tests/examples** — empty_object vs block_empty, linear_struct vs linear_field_move, linear_list vs linear_list_move
4. **Add allocation effect** section to VISION.md
5. **Add size/memory taxonomy** to VISION.md
6. **Write new tests** for allocation-as-effect
7. **Update PLAN.md** design decisions log
