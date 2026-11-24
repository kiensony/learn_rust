# task-059 — Checkpoint: exhibit caption

The first line of an exhibit label is its catalog number from the first argument, printed as `Item #number`. Print the complete supplied `caption.txt` immediately below it, preserving the file's final newline. A missing number leaves just `Item #` on line one.

From the repository directory, open `tasks/task-059/` and edit `src/main.rs`. The example commands below run inside that task directory. Try the task independently using your notes and the documentation. Compare your prediction with the test results before opening a hint.

```sh
cargo test --manifest-path tasks/task-059/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-059/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- 17` produces standard output `"Item #17\nPainted clay\n"`.
- Running `cargo run -- 09` produces standard output `"Item #09\nPainted clay\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-059/Cargo.toml
```
