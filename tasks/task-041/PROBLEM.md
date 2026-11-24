# task-041 — Print a title in columns

A two-column text file needs a header. Write `name` and `score` separated by one tab character, then finish the line. In an ordinary string literal, a backslash followed by t represents one tab.

From the repository directory, open `tasks/task-041/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-041/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-041/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"name\tscore\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-041/Cargo.toml
```
