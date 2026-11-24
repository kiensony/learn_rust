# task-023 — Send a warning to its own stream

A pipeline consumes the line `42` from standard output. A person needs to see `using fallback` on standard error. Use the two output streams so the warning cannot enter the pipeline.

From the repository directory, open `tasks/task-023/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-023/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-023/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"42\n"` and standard error `"using fallback\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-023/Cargo.toml
```
