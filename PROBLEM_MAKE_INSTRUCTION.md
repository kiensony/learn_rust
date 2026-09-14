# PROBLEM_MAKE_INSTRUCTION.md

For agents (and humans using an AI) that **author** a new `task-NNN`.
Do not solve the task. Do not put a working answer in `src/lib.rs`.

## When this mode applies

User asks to create / extend / generate `task-xxx` (or the next number).
Read this file + existing `tasks/task-*/PROBLEM.md` so the new task does not repeat an old API.

## Pick the issue from N

`N` = decimal task number. `NNN` = 3-digit folder (`7` → `task-007`, crate `task_007`).
Higher N = harder, new concept, not a reskin of an earlier task.

| Band | N | Concept to force | Allowed surface | Ban |
|------|---|------------------|-----------------|-----|
| A | 1–3 | strings, `if`/`match`, `Result`/`Option` with exact error text | free fns | ownership puzzles, traits |
| B | 4–6 | move, `&`/`&mut`, structs + `impl` | owned vs borrowed args | `Rc`, threads, generics-heavy |
| C | 7–9 | enums, `BTreeMap`/`Vec`, traits + simple generics | exhaustive match | `unsafe`, lifetimes beyond `'a` on two refs |
| D | 10–12 | iterators, explicit lifetimes, modules (`mod` + `pub use`) | adapters, `split_once` | extra crates |
| E | 13–15 | `Box`, `Rc`, `RefCell`/`Cell` (safe interior mut) | std smart pointers | raw pointers, `unsafe` |
| F | 16–18 | threads, `mpsc`, `Mutex`/`RwLock` | join, send values | `unsafe`, FFI, rolling your own sync |
| G | 19+ | multi-type design: custom error enum + iterator + collection | compose prior bands | new crates, `unsafe`, macros beyond `derive` |

If N falls in a band that already has a task with the same verb (parse, count, stack, greet): pick a **different domain** (dates, logs, spans, queues, votes, paths).

One primary concept per task. A hard test may mix one earlier concept.

## Files you must write

```
tasks/task-NNN/
  Cargo.toml          # package name task_NNN, edition 2021, no deps
  PROBLEM.md          # spec only
  REVIEW.md           # rubric
  src/lib.rs          # signatures + stubs, not a solution
  tests/public.rs     # 2–4 obvious cases
  tests/hidden.rs     # proof + edge/hard cases
```

Add `"tasks/task-NNN"` to the workspace `Cargo.toml` `members` list.

## PROBLEM.md shape

```
# task-NNN — <short name>

<one paragraph: what to build>

```rust
<exact public signatures>
```

- bullets: behavior, exact error strings, tie-breaks
- "No unsafe. Std only."
```

No hints that are the algorithm. No sample full implementation.

## src/lib.rs stub rules

- Public types/fns named as in PROBLEM.md.
- Bodies compile and are wrong or empty (`None`, `Vec::new()`, `Err(...)`, `0`).
- Do **not** include `TRACK_PROOF` or the handwritten comment. The student copies those from `INSTRUCTION.md`.
- No `todo!()` if it makes `cargo test` skip compile — compile the crate; tests may fail to compile until proof exists (that is intended).
- No `unsafe`. No extra modules unless the problem is about modules.

## tests

`tests/hidden.rs` must:

```rust
use task_NNN::TRACK_PROOF;

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | N);
}
```

Plus hard cases: empty, max/min ints, ties, extra separators, overflow, unsorted input, unicode if the spec mentions bytes vs chars.

`tests/public.rs` only easy cases from PROBLEM.md. Do not import `TRACK_PROOF` in public tests.

Do not write the student's solution in tests (no copy-paste of the algorithm as comments).

## REVIEW.md shape

Must-pass: proof comment + `TRACK_PROOF == 0x52535400 | N`, exact signatures, no `unsafe`, tests green.

Quality: the band's idiomatic tool (`checked_*`, `entry`, iterator adapters, no needless clone).

Reject: missing proof, LLM comment dump, extra public API, `unwrap` on specified `Result` paths.

## Agent checklist before finishing

1. N and crate name match folder.
2. Workspace member added.
3. Concept matches the band table.
4. Hidden tests include proof + at least 3 edge cases.
5. Stub is not a correct solution.
6. PROBLEM.md does **not** mention `TRACK_PROOF` or the handwritten comment.

## Prompt contract (what the user should paste)

See `INSTRUCTION.md` § Create a problem with AI. Follow that prompt; do not also emit a solved `lib.rs`.
