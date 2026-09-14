# AGENTS.md

This repo is a sequential Rust course. Two agent modes: **review/coach** and **problem author**.

- Review/coach: humans solve tasks. Do not replace student code with a finished solution unless they explicitly ask for a walkthrough *after* attempting the task.
- Problem author: user asks to create `task-xxx` / the next task. Follow `PROBLEM_MAKE_INSTRUCTION.md`. Do not ship a working solution in `src/lib.rs`.

## Layout

- `INSTRUCTION.md` — human rules, proof token, how to ask AI for a **new problem**.
- `PROBLEM_MAKE_INSTRUCTION.md` — how to choose the issue for `task-NNN` and which files to write.
- `tasks/task-NNN/` — one problem. Harder as N grows.
- Per task: `PROBLEM.md`, `REVIEW.md`, `src/lib.rs` (student), `tests/` (do not edit while solving).

## Commands

```bash
cargo test -p task_NNN
cargo test
```

Crate name: `task-001` → `task_001`.

## Hard invariants

- Reject `unsafe` in student code. No exceptions.
- Reject extra dependencies.
- Reject edits to `tests/` or REVIEW/PROBLEM files by the student unless they are the course author.
- Do not implement the solution into `src/lib.rs` during a review pass. Point at failing tests and principles.

## Review procedure

1. Read `tasks/task-NNN/REVIEW.md` and `PROBLEM.md`.
2. Read `src/lib.rs` only for that task.
3. Confirm handwritten proof from INSTRUCTION.md:
   - First line is `// rust-track: handwritten task-NNN`
   - `pub const TRACK_PROOF: u32 = 0x52535400 | N` with N = task number
4. Run `cargo test -p task_NNN`.
5. Grade against REVIEW.md. Output: PASS / FAIL, then a short list of defects.

## Reject as likely AI dump

Fail the review (even if tests pass) when several of these appear:

- Missing proof comment or wrong `TRACK_PROOF`
- Boilerplate comments that narrate the code
- Unsolicited rustdoc on every public item
- `unwrap`/`expect` where the signature uses `Option`/`Result`
- Needless `clone`, `to_owned`, `to_string` to skip ownership
- `#[allow(dead_code)]`, `#[allow(unused)]`, clippy allows as a blanket
- Extra public API not in PROBLEM.md
- Emoji, "As an AI", "Certainly", "Let's implement"
- `todo!` / `unimplemented!`

One small style nit is not an AI dump. A cluster is.

## Coaching mode

If the student is stuck: ask what they tried, name the Rust concept (ownership, match, Result), give a tiny hint. Do not paste a full passing `lib.rs`.

## Problem author mode

Trigger: "create task-N", "next task", "generate a problem", "make hidden tests".

1. Read `PROBLEM_MAKE_INSTRUCTION.md` (required).
2. List existing `tasks/task-*` and their titles. Do not clone an old API.
3. Map N to the band table in that file. That band **is** the issue you must create (ownership vs traits vs threads, etc.).
4. Write the full file set + workspace member. Stubs only in `src/lib.rs`.
5. Hidden tests include `TRACK_PROOF == 0x52535400 | N`.
6. Keep `TRACK_PROOF` and the handwritten comment out of `PROBLEM.md`.
7. Stop. Do not also solve the task "to verify" inside `src/lib.rs`. If you need to self-check, keep the reference solution off-repo or delete it.
