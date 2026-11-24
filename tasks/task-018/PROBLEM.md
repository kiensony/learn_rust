# task-018 — Show a Windows-style path

A help screen must show the example path `C:\course\notes` literally on every operating system. This program prints a path as text. A raw string begins with `r` before the opening quote and preserves backslashes.

From the repository directory, open `tasks/task-018/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-018/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-018/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"C:\\course\\notes\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-018/Cargo.toml
```
