# task-056 — Checkpoint: trailhead card

A ranger enters a trail and a day as two arguments. Display exactly three lines: `TRAIL CARD`, the trail in square brackets and the day in parentheses. Missing arguments leave the corresponding brackets empty. Ignore extra arguments.

From the repository directory, open `tasks/task-056/` and edit `src/main.rs`. The example commands below run inside that task directory. Try the task independently using your notes and the documentation. Compare your prediction with the test results before opening a hint.

```sh
cargo test --manifest-path tasks/task-056/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-056/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- Ridge Tue` produces standard output `"TRAIL CARD\n[Ridge]\n(Tue)\n"`.
- Running `cargo run -- "West Gate" Mon` produces standard output `"TRAIL CARD\n[West Gate]\n(Mon)\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-056/Cargo.toml
```
