# TODO

Immediate tasks for the current/next agent.

## Current State

**332 tests exist but ALL need rewriting** — the syntax has fundamentally changed.

Major design revision completed 2026-02-09. See `docs/DESIGN-REVIEW.md` for full analysis and `docs/VISION.md` for the updated language design.

## ACTIVE: Design Walkthrough with Max (Spec Mode)

Walking through design contradictions one at a time. **Resolved so far:**

1. ✅ **`{}` is always an object** — blocks after keywords use `{}`, standalone multi-statement uses `()`. Try smart disambiguation first, fall back to `()`.
2. ✅ **Comments use `#`** — PLAN.md was wrong, now fixed.
3. ✅ **Postfix juxtaposition replaces `|` pipe** — `x f` = `f(x)`, `x f(y)` = `f(x, y)`.
4. ✅ **Implicit first argument** — functions have an implicit pipeline arg from raro. Explicit params are for extra args only.
5. ✅ **Pipeline arg = runtime data** — explicit params can be lifted to earlier stages.
6. ✅ **Effect invocation** — function-call style, not `send X with Y`.
7. ✅ **Allocation as effect** — `alloc` effect for heap allocation.
8. ✅ **Size taxonomy** — known/unknown × static/dynamic.

**Still to walk through with Max:**
- Handler syntax consolidation (6 different forms → 1 canonical form)
- Linear struct field access (borrow vs move)
- Linear list indexing
- `{ x }` disambiguation rule
- `as` for naming implicit arg
- Postfix `.field` vs `.[n]` confirmation
- Defer capture semantics
- Chained comparisons

## After Design Walkthrough

1. **Rewrite all 332 example `.later` files** to use new syntax (postfix application, `#` comments, etc.)
2. **Rewrite all 332 tests in `tests/e2e.rs`** to match new examples
3. **Add new tests** for:
   - Allocation effect (no-alloc functions, arena handlers)
   - Size tracking (stack vs heap, known vs unknown)
   - Implicit first argument
   - Postfix application chains
   - `()` for multi-statement expressions
   - `?` postfix error propagation
   - `await` as postfix function

## Key Files

- `docs/VISION.md` — **UPDATED** language design with postfix application, alloc effect, size taxonomy
- `docs/PLAN.md` — **UPDATED** implementation roadmap with design decisions
- `docs/DESIGN-REVIEW.md` — full analysis of contradictions (some still open)
- `tests/e2e.rs` — 332 E2E tests (**ALL NEED REWRITING**)
- `examples/*.later` — 332 example programs (**ALL NEED REWRITING**)
- `src/main.rs` — stub (22 lines, just reads file and exits)
- `raro.ignore/` — reference copy of raro (postfix application, running arithmetic)
- `kal.ignore/` — reference copy of kal (effects, symbols, comment system)

## Handover Notes

This session was a design reconciliation walkthrough with Max. The major outcome is that `later` now adopts raro's postfix function application model (implicit first arg, juxtaposition instead of `|` pipe). This is a fundamental syntax change that affects every single test and example file.

The next agent should:
1. Read `docs/VISION.md` — it's the source of truth
2. Continue the design walkthrough if Max wants (remaining questions listed above)
3. When design is settled, rewrite all tests and examples
4. Reference `raro.ignore/` and `kal.ignore/` for the ancestor language designs
