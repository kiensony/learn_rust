# task-027 — Keep the argument out of the template

An announcement accepts a message as its first argument. Print that message verbatim on one line, including braces or percent signs. With no argument, print an empty line.

From the repository directory, open `tasks/task-027/` and edit `src/main.rs`. The example commands below run inside that task directory. Reproduce the symptom first. Use the first diagnostic or failing example to isolate the responsible statement. Make one small repair, then rerun the same case and the acceptance tests.

```sh
cargo test --manifest-path tasks/task-027/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-027/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- "Board now"` produces standard output `"Board now\n"`.
- Running `cargo run -- 23%` produces standard output `"23%\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-027/Cargo.toml
```
