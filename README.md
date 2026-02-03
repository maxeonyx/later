# later

A programming language where cleanup is not an afterthought.

**later** explores:
- Linear types for guaranteed resource cleanup
- Cancellation safety (all code is cancellable)
- Structured concurrency with task hierarchies
- Effects for control flow (errors, generators, cancellation)
- Fallible cleanup that the type system acknowledges
- Multistage programming (build → startup → runtime)
- Left-to-right evaluation (BODMAS is buried 🪦)

## Status

Early development. See [docs/VISION.md](docs/VISION.md) for the design and [docs/PLAN.md](docs/PLAN.md) for implementation status.

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Running

```bash
cargo run -- examples/int_literal.later
```

## License

TBD
