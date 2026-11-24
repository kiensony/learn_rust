# task-058 — Checkpoint: identify this release

Release this package as version `1.3.0`. Make the binary print its Cargo package name, a slash and its Cargo version with a single final newline. Both values must come from Cargo at build time.

From the repository directory, open `tasks/task-058/` and edit `Cargo.toml`, `src/main.rs`. The example commands below run inside that task directory. Try the task independently using your notes and the documentation. Compare your prediction with the test results before opening a hint.

```sh
cargo test --manifest-path tasks/task-058/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-058/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"task_058/1.3.0\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-058/Cargo.toml
```
