# INSTRUCTION.md — Rust Track

Read this before any task. Review agents assume you did.

## Setup

Need: Rust stable (`rustc`, `cargo`).

```bash
cd rust-track
cargo test -p task_001   # run one task
cargo test               # run all
```

Work only in that task's `src/lib.rs`. Do not edit `tests/`.

## Flow

1. Open `tasks/task-NNN/PROBLEM.md`.
2. Implement in `tasks/task-NNN/src/lib.rs`.
3. `cargo test -p task_NNN`.
4. Ask a review agent to grade using `REVIEW.md` + root `AGENTS.md`.
5. Pass review before `task-NNN+1`. Higher N = harder.

Crate names use underscores: `task-001` → `-p task_001`.

## Rules

- Safe Rust only. `unsafe` is an automatic fail.
- Std only. No extra crates.
- No `todo!`, `unimplemented!`, or leftover `panic!("not implemented")`.
- Match names, signatures, and error strings in PROBLEM.md exactly.
- Do not weaken tests.

## Handwritten proof (required)

Every `src/lib.rs` must contain both:

1. First line exactly (NNN = folder number, zero-padded):

```rust
// rust-track: handwritten task-NNN
```

2. This constant, where `N` is the decimal task number (`task-001` → `1`):

```rust
pub const TRACK_PROOF: u32 = 0x52535400 | N;
```

Examples: task-001 → `0x52535400 | 1`, task-010 → `0x52535400 | 10`.

Hidden tests check the constant. Review agents check the comment. Missing either = reject.

## Anti-AI review

If you paste PROBLEM.md into an AI and paste the output back, review will usually fail. Agents reject:

- Missing proof comment or `TRACK_PROOF`
- `unsafe`, extra crates, `#[allow(...)]` to hide warnings
- Comments that restate each line
- Stock rustdoc on every item when the problem did not ask
- `clone()` / `.to_string()` to dodge borrowing
- `unwrap()` / `expect()` on library paths the problem wants `Option`/`Result`
- Helpers or APIs the problem did not specify
- Emoji in code or comments
- Identical phrasing to common LLM boilerplate (`// Function to ...`, `// Let's implement`)

Write the **solution** yourself. Use docs and the compiler, not generated dumps.

## Create a problem with AI

Allowed: use AI to **author** the next exercise (spec + stubs + tests).
Not allowed: use AI to fill `src/lib.rs` for a task you are solving.

### Steps

1. See the last folder in `tasks/` (e.g. `task-010`). Next is `task-011`.
2. Open `PROBLEM_MAKE_INSTRUCTION.md`. Note the **band** for that N (what concept the task must teach).
3. Paste the prompt below into an agent that can write files. Do not add "also give me the solution".
4. Check the new folder: `PROBLEM.md`, `REVIEW.md`, stub `src/lib.rs`, `tests/public.rs`, `tests/hidden.rs`, workspace member.
5. Confirm `PROBLEM.md` does not mention `TRACK_PROOF`. Confirm hidden tests do.
6. Solve it by hand. Then review with `AGENTS.md`.

### Prompt to paste

```
You are in problem-author mode for this Rust track.

Read AGENTS.md and PROBLEM_MAKE_INSTRUCTION.md.
Create task-NNN (use the next unused number).

Rules:
- Choose the issue from the band table for N. Do not copy an existing task's API.
- Write PROBLEM.md, REVIEW.md, stub src/lib.rs, tests/public.rs, tests/hidden.rs, Cargo.toml.
- Add the crate to the workspace members.
- Stub must compile as a lib but must not be a correct solution.
- Hidden tests must check TRACK_PROOF == 0x52535400 | N and include hard cases.
- Do not put TRACK_PROOF or the handwritten comment in PROBLEM.md.
- Do not write a working solution in src/lib.rs.
- No unsafe. Std only.
```

Replace `NNN` with the next number. If you want a theme (logs, votes, paths), add one line: `Domain: …`. Do not ask the AI for the answer key.
