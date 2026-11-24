# task-054 — Move a repeated prompt into a shared file

Two commands display the same prompt. Move the complete line `Select a drawer.` (including one final newline) into `prompt.txt`. Make the main binary and the secondary binary `again` both display that file exactly.

From the repository directory, open `tasks/task-054/` and edit `prompt.txt`, `src/main.rs`, `src/bin/again.rs`. The example commands below run inside that task directory. The package's default binary is the main program. Use `cargo run --bin again` for the other one.

```sh
cargo test --manifest-path tasks/task-054/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-054/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Select a drawer.\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-054/Cargo.toml
```
