# Koka Effect Handlers — Reference

Extracted from [koka-lang.github.io/koka/doc/book.html](https://koka-lang.github.io/koka/doc/book.html)

## Effect Declaration

```koka
# Single operation shorthand (effect name = operation name)
effect ctl raise( msg : string ) : a

# Multi-operation effect
effect state<a>
  fun get() : a
  fun set( x : a ) : ()

# Generator-style
effect yield
  ctl yield( i : int ) : bool
```

## Operation Kinds

- **`fun`** — tail-resumptive: resumes exactly once, automatically. More efficient (no stack capture).
- **`ctl`** — general control: gives a first-class `resume` function. Can resume zero, once, or many times.
- **`val`** — dynamically bound value.

## Handler Syntax

### Full form
```koka
with handler
  ctl raise(msg) 42
  return(x) x + 1       # transform the final result
8 + safe-divide(1,0)
```

### Single-operation shorthand
```koka
# These are equivalent:
with ctl raise(msg) 42           # shorthand
with handler { ctl raise(msg) 42 }  # expanded

with fun ask() 21                # tail-resumptive shorthand
```

### `with` scopes the handler over the rest of the block
```koka
fun ask-const() : int
  with ctl ask() resume(21)
  add-twice()
```

### Handlers as values (abstractable)
```koka
fun emit-console( action )
  with fun emit(msg) println(msg)
  action()

# Then use it:
fun ehello-console2() : console ()
  with emit-console
  ehello()
```

## Resume

`resume` is implicitly bound inside `ctl` operations. It resumes back to the call-site:

```koka
# Resume with a value
with ctl ask() resume(21)

# Resume conditionally
with ctl ask()
  count := count + 1
  if count <= 1 then resume(42) else 0

# No resume = the handler returns directly (like throw/raise)
with ctl raise(msg) 42   # returns 42, doesn't resume
```

## Common Patterns

### Exception (no resume)
```koka
effect ctl raise( msg : string ) : a

fun raise-maybe( action ) : maybe<a>
  with handler
    return(x)      Just(x)
    ctl raise(msg) Nothing
  action()
```

### State (fun = tail-resumptive)
```koka
effect state<a>
  fun get() : a
  fun set( x : a ) : ()

fun state( init, action )
  var st := init
  with handler
    fun get()  st
    fun set(i) st := i
  action()
```

### Generator (ctl = may resume multiple times)
```koka
effect yield
  ctl yield( i : int ) : bool

fun print-elems()
  with ctl yield(i)
    println("yielded " ++ i.show)
    resume(i<=2)
  traverse([1,2,3,4])
```

### Writer/Emit (collecting)
```koka
effect fun emit( msg : string ) : ()

fun emit-collect( action ) : string
  var lines := []
  with handler
    return(x)     lines.reverse.join("\n")
    fun emit(msg) lines := Cons(msg,lines)
  action()
```

## Key Design Points

1. **`with` statement** scopes handler over "the rest of the block" — no explicit wrapping braces around the action
2. **`fun` vs `ctl`** distinguishes tail-resumptive (efficient) from general control (stack capture)
3. **`return(x)` clause** transforms the final result of the handled computation
4. **Handlers discharge effects** — the handler's type removes the handled effect from the row
5. **Handlers are first-class values** — `handler { ... }` returns a function, can be passed around
6. **Effect rows are polymorphic** — `<raise|e>` means "raise plus whatever else"
