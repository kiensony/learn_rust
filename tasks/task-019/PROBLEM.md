# task-019 — Read the first compiler message

Use `cargo check` to locate the syntax error. Repair the two statements so the screen shows `begin` and `end` on separate lines.

From the repository directory, open `tasks/task-019/` and edit `src/main.rs`. The example commands below run inside that task directory. Reproduce the symptom first. Use the first diagnostic or failing example to isolate the responsible statement. Make one small repair, then rerun the same case and the acceptance tests.

```sh
cargo test --manifest-path tasks/task-019/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-019/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"begin\nend\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-019/Cargo.toml
```
