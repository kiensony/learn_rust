# task-042 — Read the release notes from a different directory

A short announcement lives in `assets/news.txt`. Display that file exactly between an opening line `---` and a closing line `---`. The file already contains its final newline.

From the repository directory, open `tasks/task-042/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-042/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-042/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"---\nNew timetable\n---\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-042/Cargo.toml
```
