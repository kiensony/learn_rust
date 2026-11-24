# task-024 — End a prompt at the cursor

Print `Name> ` including one final space and leave the cursor on the same line. This exercise prints a prompt only.

From the repository directory, open `tasks/task-024/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-024/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-024/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Name> "`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-024/Cargo.toml
```
