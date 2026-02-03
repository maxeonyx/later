# Later: Implementation Plan

This document tracks the implementation plan. It is updated as work progresses.

## Current Status

**Phase 0: Project Setup** - COMPLETE
**Phase 1: Basic Parsing** - Tests written, implementation pending

47 E2E tests written, all failing with "not yet implemented". Ready for implementation.

## Phases

### Phase 0: Project Setup
- [x] Initialize repo
- [x] Create VISION.md
- [x] Create PLAN.md
- [x] Set up test infrastructure (E2E test harness)
- [x] Create AGENTS.md with handover instructions
- [x] First failing tests (47 tests across all phases)

### Phase 1: Basic Parsing
- [ ] Lexer for basic tokens (numbers, identifiers, operators)
- [ ] Kebab-case identifier support
- [ ] Basic expressions (literals, binary ops)
- [ ] `let` bindings
- [ ] `as` inline bindings
- [ ] `fn` definitions
- [ ] Object literals `{ key: value }`
- [ ] List literals `[a, b, c]`
- [ ] Spread operators `...`
- [ ] Pipe operator `|`

### Phase 2: Linear Type Checking
- [ ] Track ownership of values
- [ ] Error on unused linear values
- [ ] Error on use-after-consume
- [ ] Allow explicit `drop` for linear values
- [ ] Borrowing syntax (`&`) and semantics

### Phase 3: Basic Interpreter
- [ ] Evaluate expressions
- [ ] Variable scoping
- [ ] Function calls
- [ ] Object/list operations

### Phase 4: Cancellation Infrastructure
- [ ] Thread-local cancellation flag
- [ ] Cancellation point insertion (conceptual - in interpreter first)
- [ ] Cancellation effect type
- [ ] Cleanup on cancellation

### Phase 5: Effect System
- [ ] `send` expression
- [ ] `handle` expression
- [ ] Built-in effects: Cancel, Error, Yield
- [ ] Effect propagation
- [ ] `continue with` for resumption

### Phase 6: Structured Concurrency
- [ ] `spawn` expression
- [ ] Task hierarchy tracking
- [ ] `all` and `race` combinators
- [ ] `await` expression
- [ ] Cancellation propagation to children

### Phase 7: Fallible Cleanup
- [ ] Cleanup expressions
- [ ] Cleanup failure handling
- [ ] Cleanup ordering guarantees

### Phase 8: Memory Size Tracking
- [ ] Size annotations on types
- [ ] Upward size propagation
- [ ] Static size computation

### Phase 9: Multistage Execution
- [ ] `@comptime` annotation
- [ ] `@startup` annotation
- [ ] Residual program generation
- [ ] Config ingestion at startup stage

### Phase 10: Compilation
- [ ] Bytecode or IR design
- [ ] Native code generation
- [ ] WASM target

## Design Decisions Log

Decisions made during implementation:

1. **Cancellation mechanism**: Thread-local flag checking. Clean, portable, works on WASM. Benchmark exotic alternatives (page faults, code patching) only if needed.

2. **Linear by default**: Unlike Rust's affine types (can drop implicitly), Later's linear types require explicit consumption. This forces cleanup to be defined.

3. **File extension**: `.later`

4. **Operator precedence**: Following Raro's "running arithmetic" style, operators are left-to-right with equal precedence. Parentheses override when needed.

5. **Test strategy**: Black-box E2E tests are primary. Run the binary, check stdout/stderr.

## Open Questions

- Exact syntax for cleanup blocks (with? defer? inline?)
- How to handle cleanup-of-cleanup failures (turtles all the way down?)
- Syntax for mutable references during borrow
- How stages interact with the type system
- Should `symbol()` be a built-in or effect?
