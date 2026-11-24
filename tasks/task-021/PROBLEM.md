# task-021 — Label two arguments

A timetable accepts a place as its first argument and a day as its second. Print one line in the form `place / day`. When either value is missing, leave its side empty. The argument reader is supplied.

From the repository directory, open `tasks/task-021/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-021/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-021/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- Pier Monday` produces standard output `"Pier / Monday\n"`.
- Running `cargo run -- Hill Friday` produces standard output `"Hill / Friday\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-021/Cargo.toml
```
