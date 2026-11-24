# task-022 — Make braces appear on a help card

The command manual illustrates a placeholder. Print the literal line `open {file}`. Inside a format string, two consecutive opening braces produce one visible opening brace. The same rule applies to closing braces.

From the repository directory, open `tasks/task-022/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-022/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-022/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"open {file}\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-022/Cargo.toml
```
