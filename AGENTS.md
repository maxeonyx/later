# Agent Handover Instructions

## IMPORTANT: READ THIS FIRST

If you are an AI agent picking up this project:

1. **Read this file completely**
2. **Read docs/VISION.md** for the language design
3. **Read docs/PLAN.md** for current status and next steps
4. **Read docs/TODO.md** for immediate tasks

## The Prime Directive

**Keep going.** 

This is a test-first development project. Your job is to:

### In Spec Mode:
1. Pick a few tests that capture requirements
2. Make them fail in the right way
3. Leave the repo in a good state (clean commits)
4. Pick more tests and keep going
5. If you run out of context/time, hand over to the next agent

### In Build Mode:
1. Pick a failing test
2. Implement just enough to make it pass
3. Run tests, verify it passes
4. Commit
5. Pick next failing test and repeat

## Current Status

**287 failing E2E tests** - Comprehensive specification complete.

The test suite covers the entire language as described in VISION.md:
- Basic syntax (expressions, bindings, functions, objects, lists)
- Control flow (if/else, loops, patterns)
- Linear types (must-use, move semantics, borrowing)
- Effects (send, handle, generators)
- Cancellation (the core innovation - all code is cancellable)
- Structured concurrency (spawn, nursery, channels)
- Multistage (@comptime, @startup)
- Memory tracking
- Edge cases (overflow, NaN, deadlock, scope escape)
- Import/export
- Real-world patterns

## Handover Protocol

When you hand over to another agent:

1. Update docs/PLAN.md with current status
2. Update docs/TODO.md with immediate next steps
3. Commit and push your work
4. Tell the next agent:
   - "Read AGENTS.md first"
   - "Keep going"
   - "Pass these instructions to your successor"

**The chain must not break.** Every agent tells the next agent to keep going and to pass it on.

## What This Project Is

**later** is a programming language where cleanup is not an afterthought.

Core innovations:
1. **Linear types by default** - Values must be consumed exactly once
2. **Cancellation everywhere** - All code has cancellation points compiled in
3. **Structured concurrency** - Tasks form a hierarchy, cancellation propagates
4. **Effects for control flow** - Errors, generators, cancellation are all effects
5. **Fallible cleanup** - Cleanup can fail, and the type system knows it
6. **Multistage execution** - Build → Startup → Runtime

See docs/VISION.md for the full picture.

## Mode Rules

### Spec Mode
- Write failing tests that capture requirements
- Do NOT write implementation code (only test code)
- Tests ARE the specification
- Show the user the failures

### Build Mode
- Make failing tests pass
- Implement minimal code to pass each test
- Do NOT add features without tests

## Code Style

- Rust for implementation
- Tests in `tests/` directory for black-box E2E tests
- Example programs in `examples/*.later`
- Follow existing patterns in the codebase
- Keep it minimal - less code is better

## Test Strategy

Priority order:
1. **Black-box E2E tests** - Run the `later` binary, check stdout/stderr
2. **White-box E2E tests** - Call internal APIs, test across components
3. **Unit tests** - For complex pure logic

The E2E test harness is in `tests/e2e.rs`. It:
- Runs `./target/debug/later examples/foo.later`
- Checks exit code, stdout, stderr
- Uses `expect_output(filename, expected)` and `expect_error(filename, expected_substring)`

## File Structure

```
later/
  src/
    main.rs       # Entry point (reads file, runs interpreter)
    lib.rs        # Library root (currently empty)
  tests/
    e2e.rs        # 218 E2E tests
  examples/
    *.later       # 150+ example programs (test inputs)
  docs/
    VISION.md     # Language design document
    PLAN.md       # Implementation roadmap
    TODO.md       # Immediate next tasks
```

## Suggested Implementation Order

1. **Lexer** - Tokenize source into stream
2. **Parser** - Build AST from tokens  
3. **Interpreter** - Evaluate AST (tree-walking initially)
4. **Linear checker** - Track ownership, error on violations
5. **Effects** - Implement effect handlers
6. **Concurrency** - Task spawning and structured joins
7. **Multistage** - Compile-time evaluation

Start with the simplest tests:
- `empty.later` - empty file produces empty output
- `int_literal.later` - `42` prints `42`
- `add.later` - `2 + 3` prints `5`

## Syntax Reference (from examples)

```later
// Comments
let x = 42                    // Let binding
let mut y = 0                 // Mutable binding
fn add(a, b) { a + b }        // Function
fn double(x) x * 2            // Single-expression function
{ a: 1, b: 2 }                // Object
[1, 2, 3]                     // List
x | f                         // Pipe
if cond { a } else { b }      // Conditional
loop { break }                // Loop
defer { cleanup }             // Cleanup
spawn { work } as task        // Task
send effect with value        // Effect
handle { } effect v { }       // Handler
```

## Remember

- Commit and push frequently
- Tests before implementation
- Keep going
- Pass it on
