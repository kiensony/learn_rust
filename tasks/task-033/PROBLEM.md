# task-033 — Include the shipped message

The maintainer keeps a one-line message in `message.txt` in the task directory. Include its full contents at compile time and print them exactly, including its final newline. Keep that file as the single source of the text.

From the repository directory, open `tasks/task-033/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-033/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-033/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Gates open at nine.\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-033/Cargo.toml
```
