# TODO

Immediate tasks for the current/next agent.

## Current State

**326 failing E2E tests** - Comprehensive specification complete.

The test suite now captures the full language vision as described in VISION.md. All tests fail with "later: not yet implemented" which is the correct baseline failure.

## Spec Mode Complete - Ready for Build Mode

The specification phase is essentially complete. The test suite covers:

- Basic expressions and arithmetic
- Booleans, comparisons, unary operators
- Let bindings, mutability, shadowing
- Functions (named, anonymous, closures, higher-order)
- Objects and lists (literals, access, operations, spread)
- Control flow (if/else, loop, while, break, continue)
- Pattern matching (destructuring, spread, wildcard)
- Strings (literals, escapes, interpolation, operations)
- Pipe operator
- Linear types (must-use, move, borrow, in aggregates)
- Effects (send, handle, continue, generators, state)
- Cancellation (core feature - flags, cleanup, blocking)
- Structured concurrency (spawn, nursery, channels, race, all)
- Fallible cleanup
- Multistage (@comptime, @startup)
- Memory/size tracking
- Import/export
- Error messages
- Real-world patterns

## Next Steps (Build Mode)

Switch to build mode and implement:

### Phase 1: Lexer
Make these tests pass first:
- `test_empty_file` - Handle empty input
- `test_integer_literal` - Tokenize integers
- `test_boolean_true/false` - Tokenize keywords

### Phase 2: Parser  
- Parse literals into AST
- Parse binary expressions
- Parse let bindings

### Phase 3: Interpreter
- Evaluate integer literals
- Evaluate arithmetic
- Evaluate comparisons
- Evaluate if/else
- Evaluate let bindings

### Phase 4: Functions
- Parse function definitions
- Parse function calls
- Implement closures

### Phase 5: Linear Types
- Track value consumption
- Error on unused values
- Error on double use

## Optional: More Spec Tests

If continuing in spec mode, consider:
- [ ] Floats with exponents (1e10, 3.14e-5)
- [ ] Hex/binary/octal literals
- [ ] Raw strings (r"...")
- [ ] Multi-line strings (""")
- [ ] Range syntax (1..10, 1..=10)
- [ ] Match expressions
- [ ] Guard clauses in patterns
- [ ] Type aliases
- [ ] Generic types
- [ ] Traits/interfaces

## Architecture Notes

Suggested module layout for build mode:

```rust
// src/lib.rs
pub mod lexer;   // Token, Lexer
pub mod ast;     // Expr, Stmt, Pattern
pub mod parser;  // Parser
pub mod types;   // Type, LinearState
pub mod interp;  // Interpreter, Value
pub mod effects; // Effect, Handler
pub mod tasks;   // Task, Nursery
pub mod stages;  // Stage, Residual
```

## Open Design Questions

Resolved in examples:
- Comments: `//` line comments
- Booleans: `true`, `false`, `and`, `or`, `not`
- Cleanup: `defer { }` blocks
- Pipe: `x | f` calls `f(x)`

Still open:
- Exact defer capture semantics (by value or by reference?)
- Shallow vs deep handler default
- Pipe partial application: `x | f(y)` means what exactly?
- How do linear types appear in function signatures?

## Blocked

(Nothing currently blocked)
