# task-046 — Avoid printing the executable path

The ticket machine must print the first user argument, or an empty line when none was supplied. Find why it currently prints a path. Keep Rust's existing argument iterator.

From the repository directory, open `tasks/task-046/` and edit `src/main.rs`. The example commands below run inside that task directory. Reproduce the symptom first. Use the first diagnostic or failing example to isolate the responsible statement. Make one small repair, then rerun the same case and the acceptance tests.

```sh
cargo test --manifest-path tasks/task-046/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-046/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- A12` produces standard output `"A12\n"`.
- Running `cargo run -- B34 C56` produces standard output `"B34\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-046/Cargo.toml
```
