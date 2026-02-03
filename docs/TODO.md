# TODO

Immediate tasks for the current/next agent.

## Current State

**190 failing E2E tests** - Comprehensive test suite covering the full language vision.

The test suite now covers:
- Basic syntax (expressions, bindings, functions, objects, lists)
- Control flow and patterns
- Linear types and borrowing
- Effects and handlers
- Structured concurrency
- Cancellation (core innovation)
- Multistage execution
- Memory tracking
- Import/export
- Real-world usage patterns

## Now (Spec Mode - Optional Additional Tests)

The test suite is comprehensive. Optional additions:

- [ ] Float/decimal numbers
- [ ] Raw strings (no escape processing)
- [ ] Multi-line strings
- [ ] Binary/hex literals
- [ ] Bitwise operators
- [ ] Range syntax (`1..10`)
- [ ] For loops (sugar for iterator + handle)
- [ ] Match expressions
- [ ] Guard clauses
- [ ] Doc comments
- [ ] WASM-specific tests

## Now (Build Mode - Start Implementation)

Ready to switch to build mode. Recommended order:

### Week 1: Core Parsing & Evaluation
1. Lexer - tokenize source into token stream
2. Parser - build AST from tokens
3. Interpreter - evaluate simple expressions
4. Tests passing: `empty`, `int_literal`, `bool_*`, `add`, `sub`, `mul`, `div`

### Week 2: Bindings & Functions
1. Let bindings, variables, scopes
2. Function definitions and calls
3. Objects and lists
4. Tests passing: `let_*`, `fn_*`, `object_*`, `list_*`

### Week 3: Control Flow & Patterns
1. If/else expressions
2. Loops (loop, break, continue)
3. Pattern matching
4. Tests passing: `if_*`, `loop_*`, `pattern_*`

### Week 4: Linear Types
1. Ownership tracking
2. Consume checking
3. Error on unused/double-use
4. Tests passing: `linear_*`

### Week 5+: Advanced Features
- Effects, cancellation, concurrency, multistage...

## Architecture Notes

Suggested module structure:
```
src/
  lexer.rs     - Tokenization
  ast.rs       - AST types
  parser.rs    - Parsing
  types.rs     - Type system, linear tracking
  interp.rs    - Tree-walking interpreter
  effects.rs   - Effect handling
  tasks.rs     - Structured concurrency runtime
  stages.rs    - Multistage execution
```

## Open Questions

- Defer capture semantics: capture at defer time or reference?
- Effect handler syntax: `handle {} my-effect v {}` vs `handle {} { my-effect v {} }`?
- Pipe partial application: `x | f(y)` means `f(x, y)` or `f(y)(x)`?
- Linear type syntax for return types?

## Blocked

(Nothing currently blocked)
