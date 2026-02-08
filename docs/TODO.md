# TODO

Immediate tasks for the current/next agent.

## Current State

**332 failing E2E tests** — all fail with "later: not yet implemented"

**Design review completed** — see `docs/DESIGN-REVIEW.md`

## ACTIVE: Design Reconciliation (Spec Mode)

A comprehensive design review has been done. The review found:

### Must-resolve contradictions:
1. **`{}` ambiguity** — `empty_object.later` and `block_empty.later` are the same code with different expected outputs
2. **Comment syntax** — PLAN.md says `//`, everything else says `#`. PLAN.md is wrong.
3. **Linear struct field access** — `linear_struct.later` moves fields via `.`, but `linear_field_move.later` says moving fields is an error
4. **Linear list indexing** — `linear_list.later` indexes to consume, but `linear_list_move.later` says indexing linear lists is an error
5. **Effect invocation** — some examples use `send X with Y`, others call effects like functions. Pick one.
6. **Handler syntax** — 6 different syntactic forms across examples. Needs consolidation.
7. **Effect declaration** — VISION.md uses `resume(T)` syntax, examples use simpler `: T` style

### New features to design & spec (Max requested):
8. **Allocation as an effect** — `alloc` effect for heap allocation, no-alloc code has no alloc effect
9. **Static vs dynamic memory distinction** — known-size vs unknown-size, stack vs heap
10. **Size taxonomy** — 2×2 matrix of known/unknown × static/dynamic

### Waiting on Max to review:
- `docs/DESIGN-REVIEW.md` — full analysis with resolution proposals
- Max needs to make decisions on contradictions before tests can be fixed
- After decisions, update VISION.md, fix conflicting examples/tests, add allocation tests

## Handover Notes

### What was done this session:
- Read ALL 332 example files, all tests, VISION.md, PLAN.md, TODO.md
- Identified 7 contradictions between VISION.md, PLAN.md, and examples
- Analyzed allocation-as-effect design (Max's new wishlist item)
- Wrote comprehensive design review in `docs/DESIGN-REVIEW.md`
- Did NOT modify any tests or application code yet (waiting for Max's decisions)

### What the next agent should do:
1. Read `docs/DESIGN-REVIEW.md` first
2. Ask Max which resolutions he wants for each contradiction
3. Update VISION.md with resolved decisions
4. Fix conflicting examples and tests
5. Write new tests for allocation-as-effect
6. Continue in spec mode until design is clean

### Key files:
- `docs/DESIGN-REVIEW.md` — THE design reconciliation analysis
- `docs/VISION.md` — language design (needs updating after review)
- `docs/PLAN.md` — implementation roadmap (has some stale decisions)
- `tests/e2e.rs` — 332 E2E tests (2314 lines)
- `examples/*.later` — 332 example programs (test inputs)
- `src/main.rs` — stub (22 lines, just reads file and exits)

## Optional: More Spec Tests (lower priority)

If continuing in spec mode after design reconciliation:
- [ ] Allocation effect tests (no-alloc functions, arena handlers)
- [ ] Size tracking tests (stack vs heap, known vs unknown)
- [ ] Floats with exponents (1e10, 3.14e-5)
- [ ] Hex/binary/octal literals
- [ ] Raw strings (r"...")
- [ ] Multi-line strings (""")
- [ ] Range syntax (1..10, 1..=10)

## Blocked

- Everything blocked on Max reviewing `docs/DESIGN-REVIEW.md` and making design decisions
