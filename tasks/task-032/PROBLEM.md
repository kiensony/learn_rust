# task-032 — Use Cargo's package name

The status command must identify its own package as recorded by Cargo. Display `package ` followed by the package name. Obtain the name from the compile-time environment variable `CARGO_PKG_NAME` so a rename is reflected automatically.

From the repository directory, open `tasks/task-032/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-032/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-032/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"package task_032\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-032/Cargo.toml
```
