# task-025 — Reuse a value on a badge

Read a label from the first command-line argument. Surround it with square brackets and then repeat it after a space, as in `[staff] staff`. With no label the result is `[] `.

From the repository directory, open `tasks/task-025/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-025/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-025/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- staff` produces standard output `"[staff] staff\n"`.
- Running `cargo run -- guest` produces standard output `"[guest] guest\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-025/Cargo.toml
```
