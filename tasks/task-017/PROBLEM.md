# task-017 — Make quotation marks visible

Print the exact line `Press "Enter" to continue.`. To include a double quote inside an ordinary Rust string literal, put a backslash before it.

From the repository directory, open `tasks/task-017/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-017/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-017/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Press \"Enter\" to continue.\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-017/Cargo.toml
```
