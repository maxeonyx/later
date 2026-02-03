# TODO

Immediate tasks for the current/next agent.

## Now (Spec Mode)

All initial E2E tests are written. Next agent should:

- [ ] Add more edge case tests for parsing (empty file, comments, trailing commas)
- [ ] Add tests for pattern matching/destructuring
- [ ] Add tests for `mut` keyword
- [ ] Add tests for control flow (`if`/`else`, `loop`, `break`, `continue`)

## Next (Build Mode)

When switching to build mode:

- [ ] Implement lexer (tokenize basic expressions)
- [ ] Implement parser (AST for expressions)
- [ ] Implement basic interpreter (evaluate literals and arithmetic)
- [ ] Make first tests pass

## Future Tests to Write

- [ ] Nested function calls
- [ ] Recursive functions
- [ ] Closures capturing variables
- [ ] Pattern matching in `let`
- [ ] Pattern matching in `handle`
- [ ] Multiple effects in one handler
- [ ] Timeout as structured concurrency
- [ ] Resource composition in structs
- [ ] Collection cleanup semantics

## Open Design Questions (Need Tests)

- What happens when you `break` from inside a `handle`?
- Can effects cross task boundaries?
- How do linear types interact with closures?
- What's the syntax for cleanup blocks on individual values?

## Blocked

(Nothing currently blocked)
