# Later: A Language for Graceful Cleanup

**later** is a programming language where cleanup is not an afterthought - it's intrinsic to how code composes.

## Core Principles

### 1. Linear Types by Default

Every value must be consumed exactly once. This isn't a restriction - it's a guarantee. When you acquire a resource, you *will* clean it up. The compiler ensures this.

```later
let file = open("data.txt")
// ... use file ...
file | close   // must consume - compiler error if you forget
```

### 2. Cancellation Everywhere

All code is cancellable. When a task is cancelled:
- Cancellation points (compiled into the code) check a thread-local flag
- A cancellation effect is raised
- Cleanup runs for all owned resources
- Linear types guarantee nothing is forgotten

Cancellation points are lightweight (~3 instructions) and inserted at:
- Function entry
- Loop heads  
- Before function calls

### 3. Structured Concurrency

Tasks form a hierarchy. A parent task cannot complete until all children complete. Cancellation propagates downward. This makes reasoning about concurrent resource ownership tractable.

```later
[task_a, task_b, task_c] | all | await?
// if any fails, all are cancelled, all clean up
```

### 4. Fallible Cleanup

Cleanup can fail. This is reality - disks get unplugged, networks go down. The type system acknowledges this:

```later
let file = open("data.txt")
// ...
file | close | handle {
    IoError e { 
        log("cleanup failed: {e}")
        // resource is consumed even if cleanup failed
    }
}
```

### 5. Composable Cleanup

Cleanup behavior emerges from how primitives compose:
- **Scope**: multiple resources clean up in reverse acquisition order
- **Struct**: struct cleanup composes field cleanups
- **Collection**: collection cleanup cleans up all elements
- **Task**: task cleanup includes all owned resources

### 6. Effects for Control Flow

Like Kal, effects handle errors, generators, async, and now cancellation:

```later
send cancel with reason
// handled by the runtime or an explicit handler
```

### 7. Upward-Propagating Memory Information

Types can carry information about their memory footprint. This propagates upward through composition, enabling:
- Compile-time memory allocation when sizes are static
- Startup-time allocation when sizes depend on config
- Runtime allocation as a fallback

### 8. Multistage Programming

Building is running. The program executes in stages:

1. **Build time**: produces a residual program
2. **Startup time**: ingests config, specializes further
3. **Runtime**: actual execution

Like Zig's comptime, but with arbitrary stages.

## Syntax Inspirations

Drawing from Raro and Kal:
- Kebab-case identifiers (`my-resource`)
- `as` for inline bindings (`expr as name`)
- `fn name { ... }` for functions
- `let` bindings with `mut` for mutability
- JS-like objects `{ key: value }`
- Effects: `send`, `handle`, `continue with`
- Pattern matching in bindings
- Spread operators `[...list]`, `{...obj}`

## The Paradigm Shift

In most languages, you write code and add cleanup later (maybe). In **later**, you cannot acquire a resource without cleanup being defined. The name is ironic - there is no "later" for cleanup.

## Target Platforms

- Native (primary)
- WASM (first-class support - cancellation via flag checking works here)
