# task-057 — Checkpoint: sort the two streams

A receipt writer emits the first argument on standard output and the fixed line `receipt copied` on standard error. Both end with one newline. With no argument, standard output is one empty line.

From the repository directory, open `tasks/task-057/` and edit `src/main.rs`. The example commands below run inside that task directory. Try the task independently using your notes and the documentation. Compare your prediction with the test results before opening a hint.

```sh
cargo test --manifest-path tasks/task-057/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-057/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- 123` produces standard output `"123\n"` and standard error `"receipt copied\n"`.
- Running `cargo run -- "A 7"` produces standard output `"A 7\n"` and standard error `"receipt copied\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-057/Cargo.toml
```
