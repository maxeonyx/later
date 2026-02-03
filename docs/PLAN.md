# Later: Implementation Plan

This document tracks the implementation plan. It is updated as work progresses.

## Current Status

**Phase 0: Project Setup** - COMPLETE
**Specification Phase** - COMPLETE (218 failing E2E tests)
**Implementation Phase** - NOT STARTED

All test infrastructure is in place. Tests comprehensively cover:
- Basic expressions, arithmetic, booleans, comparisons
- Control flow (if/else, loop, break, continue)
- Let bindings, as-bindings, pattern matching
- Functions (named, anonymous, recursive, mutual recursion)
- Objects and lists (literals, access, spread, destructuring)
- Mutability
- Pipe operator
- Linear types (unused, use-after-consume, conditional, in structs/lists/functions)
- Borrowing
- Closures (capture, linear restrictions)
- Effects (send, handle, continue, propagation, generators)
- Cancellation (cleanup ordering, propagation, during cleanup, flag checks)
- Structured concurrency (spawn, await, all, race, timeout, nursery, channels)
- Fallible cleanup (retry, failure handling)
- Memory size tracking (comptime, startup, bounded)
- Multistage (@comptime, @startup, stage errors)
- Strings (literals, escapes, interpolation, operations)
- Error messages (quality, line numbers, specific errors)
- Import/export
- Real-world patterns (graceful shutdown, retry, pools, parallel map)

## Phases

### Phase 0: Project Setup ✓
- [x] Initialize repo
- [x] Create VISION.md
- [x] Create PLAN.md
- [x] Set up test infrastructure (E2E test harness)
- [x] Create AGENTS.md with handover instructions
- [x] 109 failing tests across all phases

### Phase 1: Basic Parsing & Interpretation
- [ ] Lexer for basic tokens (numbers, identifiers, operators)
- [ ] Kebab-case identifier support
- [ ] Comments (`//`)
- [ ] Basic expressions (literals, binary ops)
- [ ] Booleans and comparisons
- [ ] `let` bindings with `mut`
- [ ] `as` inline bindings
- [ ] `fn` definitions (named, anonymous, single-expression)
- [ ] Object literals `{ key: value }`
- [ ] List literals `[a, b, c]`
- [ ] Property access (`.`, `[]`)
- [ ] Spread operators `...`
- [ ] Pipe operator `|`
- [ ] Control flow (`if`/`else`, `loop`, `break`, `continue`)
- [ ] Pattern matching in `let` and function params
- [ ] Trailing commas

### Phase 2: Linear Type Checking
- [ ] Track ownership of values
- [ ] Error on unused linear values
- [ ] Error on use-after-consume
- [ ] Conditional consumption (must consume in all branches)
- [ ] Linear values in structs/lists
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
- [ ] `send` expression
- [ ] `handle` expression with pattern matching
- [ ] `continue with` for resumption
- [ ] Effect propagation (unhandled effects error)
- [ ] Multiple handlers
- [ ] Nested handlers (shadowing)
- [ ] Rethrow pattern
- [ ] Built-in effects: Error, Yield
- [ ] Generator/collect pattern

### Phase 7: Structured Concurrency
- [ ] `spawn` expression
- [ ] Task hierarchy tracking
- [ ] `await` expression
- [ ] `all` combinator
- [ ] `race` combinator
- [ ] Timeout pattern
- [ ] Cancellation propagation to children
- [ ] Task owns linear resources

### Phase 8: Fallible Cleanup
- [ ] Cleanup can raise effects
- [ ] Cleanup failure handling
- [ ] Cleanup retry pattern
- [ ] Cleanup during cancellation completes

### Phase 9: Memory Size Tracking
- [ ] Size annotations on types
- [ ] Upward size propagation
- [ ] Bounded collections (MaxSize)
- [ ] Static size computation

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

## Design Decisions Log

Decisions made during implementation:

1. **Cancellation mechanism**: Thread-local flag checking. Clean, portable, works on WASM. Benchmark exotic alternatives (page faults, code patching) only if needed.

2. **Linear by default**: Unlike Rust's affine types (can drop implicitly), Later's linear types require explicit consumption. This forces cleanup to be defined.

3. **File extension**: `.later`

4. **Operator precedence**: Following Raro's "running arithmetic" style, operators are left-to-right with equal precedence. Parentheses override when needed.

5. **Test strategy**: Black-box E2E tests are primary. Run the binary, check stdout/stderr.

6. **Comments**: `//` for line comments (like Rust/JS, not `#` like Raro)

7. **Boolean keywords**: `true`, `false`, `and`, `or`, `not` (word-style like Python/Raro)

8. **Cleanup syntax**: `defer { }` blocks for cleanup (like Go, familiar pattern)

## Open Questions

- Exact semantics of `defer` - when does it run in relation to effects?
- How to handle cleanup-of-cleanup failures (turtles all the way down?)
- Syntax for mutable references during borrow
- How stages interact with the type system
- Should `symbol()` be a built-in or effect?
- String syntax (single quotes? double quotes? both?)
- How does `|` interact with method-like calls? `x | foo(y)` vs `x | foo` 
