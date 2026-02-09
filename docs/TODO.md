# TODO

Immediate tasks for the current/next agent.

## Current State

**332 tests exist but ALL need rewriting** — the syntax has fundamentally changed.

Major design revision completed 2026-02-09. See `docs/DESIGN-REVIEW.md` for full analysis and `docs/VISION.md` for the updated language design.

## ACTIVE: Crafting Cohesive Design Snippets (Spec Mode)

We're crafting **showcase snippets** that combine multiple language features cohesively, to validate the design before writing tests. This is pre-test design work.

### Blog Posts Informing the Design

Max provided these articles (all read and digested):
- Niko's "Must Move Types" — `?Drop` proposal, linear types in Rust
- Niko's "Move, Destruct, Forget" — `Forget > Destruct > Move > Pointee` trait hierarchy
- Niko's "Borrow checking without lifetimes" — origins/places instead of lifetime parameters
- Niko's "Sized, DynSized, Unsized" — hierarchical default bounds
- Niko's "Unwind considered harmful" — unwinding limits must-move types, borrow checker
- without.boats' "Asynchronous clean-up" — `poll_cancel`, `do...final`, linear types as prerequisites
- Yoshua's "Syntactic musings on fallibility" — `throws`/`throw`/`?` as effect syntax

**Key insight from the articles:** `later` resolves the problems all these posts describe by designing with linear-by-default, effect-based cancellation, and structured concurrency from day one. The snippets should demonstrate this.

### Snippets Completed

**Snippet 1: Async server with cleanup** (confirmed by Max)
```later
fn serve(addr) {
    let sock = listen(addr)
    defer { sock shutdown await }
    
    nursery {
        loop {
            let conn = sock accept await
            spawn {
                defer { conn close await }
                conn handle-request await
            }
        }
    }
}
```
- `defer` is syntactic sugar for cleanup at end of scope
- Enforcement comes from linearity, not from `defer` itself
- `nursery` scopes spawned tasks (keyword to be bikeshed)
- `await` inside `defer` works because cancellation is cooperative

**Snippet 2: Transaction with must-move semantics** (confirmed by Max)
```later
fn transfer(db, from, to, amount) {
    let tx = db begin-transaction

    with error(e) {
        tx rollback await
        throw(e)
    }

    tx execute("UPDATE accounts SET balance = balance - {amount} WHERE id = {from}") await ?
    tx execute("UPDATE accounts SET balance = balance + {amount} WHERE id = {to}") await ?
    tx commit await
}
```
- `tx` is linear — must be consumed exactly once
- `with` handler catches errors on the error path → rollback
- If no error, happy path reaches `commit`
- Compiler verifies `tx` consumed on ALL paths

### Snippets Still Needed

Ideas for remaining snippets to exercise other features:
- **Generator/iterator** — `yield` effect with `with` handler
- **Allocation effect** — no-alloc function vs alloc-requiring function, arena handler
- **Multistage** — `@comptime`/`@startup` with pipeline arg lifting
- **Cancellation propagation** — parent cancels children, cleanup runs
- **Borrowing vs moving** — linear struct field access
- **Postfix chains** — showing the raro-style data flow

### Design Decisions Made This Session

9. ✅ **Handler syntax** — Koka-inspired `with effect(args) { body }`, scopes over rest of block
10. ✅ **Resume determined by effect type** — Never (no resume), Once (default), Many (sequential), Fork (concurrent). Symmetric handler syntax for all four.
11. ✅ **`defer` for simple cleanup** — single code path, always runs. For affine+drop types.
12. ✅ **`with` handler for split paths** — when happy/sad paths differ (transactions). Handler catches effect, does cleanup, re-throws.
13. ✅ **Structured concurrency requires explicit scope** — no implicit scope in loop bodies. `nursery` (name TBD) scopes spawned tasks.

### All Design Decisions (cumulative)

1. ✅ `{}` is always an object — blocks after keywords, `()` for standalone multi-statement
2. ✅ Comments use `#`
3. ✅ Postfix juxtaposition replaces `|` pipe
4. ✅ Implicit first argument (raro-style)
5. ✅ Pipeline arg = runtime data
6. ✅ Effect invocation is function-call style
7. ✅ Allocation as effect (`alloc`)
8. ✅ Size taxonomy (known/unknown × static/dynamic)
9. ✅ Handler syntax — `with effect(args) { body }`, Koka-style, scopes rest of block
10. ✅ Resume symmetry — determined by effect type (Never/Once/Many/Fork)
11. ✅ `defer` for simple single-path cleanup
12. ✅ `with` handler for split-path cleanup (transactions)
13. ✅ Structured concurrency needs explicit scope

**Still to walk through with Max:**
- Linear struct field access (borrow vs move)
- Linear list indexing
- `{ x }` disambiguation rule
- `as` for naming implicit arg
- Postfix `.field` vs `.[n]` confirmation
- Defer capture semantics
- Chained comparisons

## Bikeshed

- **`nursery` keyword** — Max doesn't like it. Needs a better name for the structured concurrency scope. (`scope`? `taskgroup`? `pool`? TBD)
- **Postfix syntax** — current snippets use prefix-style (`fn serve(addr) { ... }`). Max wants raro-style reversed syntax eventually. Continuing with current style for now to focus on semantics.

## After Design Snippets

1. **Update VISION.md** with all new decisions (handler syntax, resume types, defer vs with, etc.)
2. **Delete all 332 stale example files** and replace with the cohesive snippets
3. **Rewrite all 332 tests in `tests/e2e.rs`** to match new examples
4. **Add new tests** for the feature interactions shown in snippets

## Key Files

- `docs/VISION.md` — language design (needs updating with session decisions)
- `docs/PLAN.md` — implementation roadmap with design decisions
- `docs/DESIGN-REVIEW.md` — full analysis of contradictions (some still open)
- `docs/koka-handlers-reference.md` — NEW: Koka handler syntax reference
- `tests/e2e.rs` — 332 E2E tests (**ALL STALE**)
- `examples/*.later` — 332 example programs (**ALL STALE**)
- `src/main.rs` — stub (22 lines, just reads file and exits)
- `raro.ignore/` — reference copy of raro
- `kal.ignore/` — reference copy of kal

## Handover Notes

This session is focused on crafting cohesive design snippets that exercise multiple features together. Max provided 7 blog posts about linear types, cancellation, effects, and sizing in Rust — these inform the design. The key insight is that `later` resolves the hard problems described in those posts by having linear-by-default types, effect-based cancellation, and structured concurrency from day one.

The next agent should:
1. Read `docs/VISION.md` and this TODO — they're the source of truth
2. Read `docs/koka-handlers-reference.md` for handler syntax reference
3. **Continue crafting snippets with Max** — use ADHD-friendly style (one at a time, wait for confirmation)
4. Snippets still needed: generators, alloc effect, multistage, cancellation propagation, borrowing, postfix chains
5. After snippets are done, update VISION.md and then rewrite tests
6. Reference `raro.ignore/` and `kal.ignore/` for ancestor designs
