# Later: Implementation Plan

This document tracks the implementation plan. It is updated as work progresses.

## Current Status

**Phase 0: Project Setup** - COMPLETE
**Specification Phase** - IN PROGRESS (major design revision underway)
**Implementation Phase** - NOT STARTED

### Design Revision (2026-02-09)

A comprehensive design reconciliation was performed. Key changes:

1. **Postfix juxtaposition replaces `|` pipe** — `5 double` means `double(5)`. No pipe character.
2. **Implicit first argument** — functions have an implicit pipeline arg (from raro). Explicit params are for extra args only.
3. **`{}` is always an object** — blocks after keywords use `{}` (keyword disambiguates). Standalone multi-statement expressions use `()`. Smart disambiguation may allow `{}` in some contexts.
4. **Comments use `#`** — not `//`. The `#` system supports doc comments, block comments, headings, shebangs.
5. **Allocation as an effect** — `alloc` effect for heap allocation. No-alloc code = no alloc effect.
6. **Size taxonomy** — 2×2 matrix of known/unknown size × static/dynamic allocation.

**All 332 existing tests need to be rewritten** to reflect the new postfix syntax and other changes.

See `docs/DESIGN-REVIEW.md` for the full analysis of contradictions found and resolved.

## Design Decisions Log

Decisions made during design:

1. **Cancellation mechanism**: Thread-local flag checking. Clean, portable, works on WASM.

2. **Linear by default**: Unlike Rust's affine types (can drop implicitly), Later's linear types require explicit consumption. But copyable types (Int, Bool, String) are freely copyable — linearity is for resources.

3. **File extension**: `.later`

4. **Operator precedence**: Raro's "running arithmetic" — left-to-right, no BODMAS. Parentheses override.

5. **Test strategy**: Black-box E2E tests are primary. Run the binary, check stdout/stderr.

6. **Comments**: `#` for line comments, `##` for doc comments, `#*...*#` for block comments. NOT `//`.

7. **Boolean keywords**: `true`, `false`, `and`, `or`, `not` (word-style like Python/Raro)

8. **Cleanup syntax**: `defer { }` blocks for cleanup (like Go).

9. **Function application**: Postfix juxtaposition. `x f` = `f(x)`. `x f(y)` = `f(x, y)`. Functions have an implicit first argument; explicit params are for non-pipeline args. From raro.

10. **Object vs block**: `{}` is always an object literal. Keyword-introduced blocks (`if`, `fn`, `loop`, etc.) use `{}` with the keyword as disambiguator. Standalone multi-statement expressions use `()`. Smart disambiguation attempted first.

11. **Pipeline arg = runtime data**: The implicit first arg is the runtime data flowing through. Explicit params can often be lifted to earlier stages (comptime/startup).

12. **Allocation as effect**: Heap allocation requires the `alloc` effect. Stack allocation of known-size values is effect-free. The compiler tracks sizes to determine what needs alloc.

13. **Effect invocation**: Effects are called like functions (not `send X with Y`). `fail("boom")` not `send fail with "boom"`.

14. **Effect declaration**: Simple return-type style: `effect ask(): Int`. Not the verbose `effect ask(): resume(Int)`.

## Remaining Design Questions

These need resolution before tests can be finalized:

- **Handler syntax consolidation** — examples show 6+ different handler forms. Need one canonical grammar.
- **Linear struct field access** — should `.field` borrow or move? Destructuring moves, dot access borrows?
- **Linear list indexing** — random index access on linear lists: borrow or error? Need special iteration/destructuring.
- **Defer capture semantics** — by value at defer time or by reference?
- **Chained comparisons** — `1 < 2 < 3` expands to `(1 < 2) and (2 < 3)` — special syntax?
- **`{ x }` ambiguity** — object shorthand or expression? Smart disambiguation rule needed.
- **`as` for naming implicit arg** — `fn process { as x; x + 1 }` — confirm this is the mechanism.
- **Postfix `.field` vs `.[n]`** — confirm index syntax uses `.[n]` not `[n]`.

## Phases

### Phase 0: Project Setup ✓
- [x] Initialize repo
- [x] Create VISION.md
- [x] Create PLAN.md
- [x] Set up test infrastructure (E2E test harness)
- [x] Create AGENTS.md with handover instructions

### Phase 0.5: Design Revision (CURRENT)
- [x] Comprehensive design review (`docs/DESIGN-REVIEW.md`)
- [x] Resolve major contradictions
- [x] Update VISION.md with postfix application, alloc effect, size taxonomy
- [ ] Resolve remaining design questions (handler syntax, linear access, etc.)
- [ ] Rewrite all 332 tests for new syntax
- [ ] Rewrite all 332 example `.later` files for new syntax

### Phase 1: Basic Parsing & Interpretation
- [ ] Lexer for basic tokens (numbers, identifiers, operators)
- [ ] Kebab-case identifier support
- [ ] Comments (`#`, `##`, `#*...*#`)
- [ ] Basic expressions (literals, binary ops)
- [ ] Postfix function application (juxtaposition)
- [ ] Implicit first argument
- [ ] Booleans and comparisons
- [ ] `let` bindings with `mut`
- [ ] `as` inline bindings
- [ ] `fn` definitions (implicit first arg, explicit extra params)
- [ ] Object literals `{ key: value }` (always object, never block)
- [ ] List literals `[a, b, c]`
- [ ] Postfix field access `.field`, index `.[n]`
- [ ] Spread operators `...`
- [ ] Control flow (`if`/`else`, `loop`, `break`, `continue`)
- [ ] Pattern matching in `let` and function params
- [ ] `()` for multi-statement expressions
- [ ] Trailing commas

### Phase 2: Linear Type Checking
- [ ] Track ownership of values
- [ ] Linearity hierarchy (linear, affine+drop, copyable)
- [ ] Error on unused linear values
- [ ] Error on use-after-consume
- [ ] Conditional consumption (must consume in all branches)
- [ ] Linear values in structs — dot access borrows, destructuring moves
- [ ] Linear values through functions (transfer, return)
- [ ] Wildcard pattern `_` for discarding
- [ ] Borrowing syntax (`&`) and semantics
- [ ] Borrow lifetime tracking

### Phase 3: Closures
- [ ] Variable capture
- [ ] Mutable capture
- [ ] Linear capture restriction
- [ ] Borrow capture

### Phase 4: Recursion
- [ ] Self-recursion
- [ ] Mutual recursion (forward references)

### Phase 5: Cancellation Infrastructure
- [ ] Thread-local cancellation flag
- [ ] Cancellation point insertion
- [ ] Cancellation effect type
- [ ] Cleanup on cancellation (`defer`)
- [ ] Cleanup ordering (reverse acquisition)

### Phase 6: Effect System
- [ ] `symbol()` built-in
- [ ] Effect declarations (`effect name(): Type`)
- [ ] Effect invocation (function-call style)
- [ ] `handle` expression
- [ ] `resume` for resumption
- [ ] `?` postfix error propagation
- [ ] Effect propagation (unhandled effects = compile error)
- [ ] Multiple handlers
- [ ] Nested handlers (shadowing)
- [ ] Built-in effects: panic, error, cancel, alloc

### Phase 7: Structured Concurrency
- [ ] `spawn` expression
- [ ] Task hierarchy tracking
- [ ] `await` (postfix)
- [ ] `all` combinator (postfix)
- [ ] `race` combinator (postfix)
- [ ] Timeout pattern
- [ ] Cancellation propagation to children
- [ ] Task owns linear resources

### Phase 8: Fallible Cleanup
- [ ] Cleanup can raise effects
- [ ] Cleanup failure handling
- [ ] Cleanup retry pattern
- [ ] Cleanup during cancellation completes

### Phase 9: Memory & Allocation
- [ ] `alloc` effect
- [ ] Size annotations on types
- [ ] Upward size propagation
- [ ] Bounded collections (MaxSize)
- [ ] Static size computation
- [ ] Stack vs heap determination

### Phase 10: Multistage Execution
- [ ] `@comptime` annotation
- [ ] `@startup` annotation
- [ ] Residual program generation
- [ ] Config ingestion at startup stage

### Phase 11: Compilation
- [ ] Bytecode or IR design
- [ ] Native code generation
- [ ] WASM target
- [ ] Cancellation point code generation
