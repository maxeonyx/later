# Agent Handover Instructions

## IMPORTANT: READ THIS FIRST

If you are an AI agent picking up this project:

1. **Read this file completely**
2. **Read docs/VISION.md** for the language design
3. **Read docs/PLAN.md** for current status and next steps
4. **Read docs/TODO.md** for immediate tasks

## The Prime Directive

**Keep going.** 

Your job is to:
1. Pick a few tests that capture requirements
2. Make them fail in the right way
3. Leave the repo in a good state (passing CI, clean commits)
4. Pick more tests and keep going
5. If you run out of context/time, hand over to the next agent

## Handover Protocol

When you hand over to another agent:

1. Update docs/PLAN.md with current status
2. Update docs/TODO.md with immediate next steps
3. Commit and push your work
4. Tell the next agent:
   - Read AGENTS.md first
   - Keep going
   - Pass these instructions to their successor

**The chain must not break.** Every agent tells the next agent to keep going and to pass it on.

## What This Project Is

**later** is a programming language focused on:
- Linear types for guaranteed cleanup
- Cancellation safety (all code is cancellable)
- Structured concurrency
- Effects for control flow
- Multistage programming

See docs/VISION.md for the full picture.

## Spec Mode Rules

This repo is being built test-first. In spec mode:
- Write failing tests that capture requirements
- Do NOT write implementation code
- Show the user the failures
- Tests ARE the specification

## Code Style

- Rust for implementation
- Tests in `tests/` directory for black-box E2E tests
- Tests in `src/` for unit tests
- Follow existing patterns in the codebase
- Keep it minimal

## Test Strategy

Priority order:
1. **Black-box E2E tests** - Run the `later` binary, check output
2. **White-box E2E tests** - Call internal APIs, test across components
3. **Unit tests** - For complex pure logic

## File Structure

```
later/
  src/
    main.rs       # Entry point
    lib.rs        # Library root
    lexer.rs      # Tokenization
    parser.rs     # Parsing
    ast.rs        # AST types
    types.rs      # Type system
    interp.rs     # Interpreter
  tests/
    *.rs          # E2E tests
  examples/
    *.later       # Example programs
  docs/
    VISION.md     # Language design
    PLAN.md       # Implementation plan
    TODO.md       # Immediate tasks
```
