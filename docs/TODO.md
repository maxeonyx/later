# TODO

Immediate tasks for the current/next agent.

## Current State

109 E2E tests written, all failing with "not yet implemented". The test suite comprehensively covers the language features described in VISION.md.

## Now (Spec Mode - Continue Adding Tests)

Consider adding tests for:

- [ ] String literals and operations
- [ ] String interpolation
- [ ] Negative list indices (Python-style?)
- [ ] Chained comparisons (`1 < x < 10`)?
- [ ] Error messages - test that errors are helpful and point to the right location
- [ ] More edge cases in pattern matching (mismatched lengths, missing keys)
- [ ] Unicode identifiers?
- [ ] Overflow behavior (big integers?)

## Now (Build Mode - Start Implementation)

When switching to build mode, start with the simplest tests first:

1. [ ] `empty.later` - handle empty file
2. [ ] `int_literal.later` - parse and print integer
3. [ ] `bool_true.later` / `bool_false.later` - parse and print booleans
4. [ ] `add.later`, `sub.later`, `mul.later`, `div.later` - binary arithmetic
5. [ ] `let_simple.later` - let bindings
6. [ ] `if_true.later` / `if_false.later` - basic conditionals

Suggested implementation order:
1. Lexer (tokenize source)
2. Parser (build AST) 
3. Interpreter (evaluate AST)
4. Linear type checker (validate ownership)

## Future Tests to Write

### Syntax Edge Cases
- [ ] Very long identifiers
- [ ] Keywords as object keys (`{ if: 1, let: 2 }`)
- [ ] Empty objects and lists in various positions
- [ ] Deeply nested structures

### Error Quality
- [ ] Syntax error messages point to correct line/column
- [ ] Type error messages name the offending variable
- [ ] Linear type errors explain what wasn't consumed

### Advanced Features
- [ ] Effect handlers that don't continue (early return)
- [ ] Nested loops with break/continue to outer
- [ ] Complex ownership through multiple function calls
- [ ] Concurrent access patterns (borrow during spawn)

## Open Design Questions (Need Tests/Discussion)

- What happens when you `break` from inside a `handle`?
- Can effects cross task boundaries?
- How do linear types interact with closures that outlive their scope?
- What's the syntax for cleanup blocks on individual values?
- How do you express "this function may perform IO effects"?

## Blocked

(Nothing currently blocked)
