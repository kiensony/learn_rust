# task-013 — Join a label and a value

A baggage tag should read `Bag #7` on a single line. Keep the two separate output calls. Use `print!` wherever the following text must stay on the same line.

From the repository directory, open `tasks/task-013/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-013/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-013/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Bag #7\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-013/Cargo.toml
```
