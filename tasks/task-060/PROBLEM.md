# task-060 — Checkpoint: diagnose a stage cue

An operator passes two arguments: a cue and a performer. The cue sheet should read `performer <- cue` on one line. A missing argument contributes the empty string. Repair the existing display, add a regression test using two different values in `tests/regression.rs`, and explain in `NOTES.md` why identical values can hide the fault.

From the repository directory, open `tasks/task-060/` and edit `src/main.rs`, `tests/regression.rs`, `NOTES.md`. The example commands below run inside that task directory. Try the task independently using your notes and the documentation. Compare your prediction with the test results before opening a hint.

```sh
cargo test --manifest-path tasks/task-060/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-060/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- enter Mina` produces standard output `"Mina <- enter\n"`.
- Running `cargo run -- wait Bo` produces standard output `"Bo <- wait\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-060/Cargo.toml
```
