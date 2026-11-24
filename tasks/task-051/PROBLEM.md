# task-051 — Close a format placeholder

Repair the malformed arrival board. It must print `Arriving: ` followed by the first argument, preserving spaces and braces inside the argument. With no argument it shows the prefix only.

From the repository directory, open `tasks/task-051/` and edit `src/main.rs`. The example commands below run inside that task directory. Reproduce the symptom first. Use the first diagnostic or failing example to isolate the responsible statement. Make one small repair, then rerun the same case and the acceptance tests.

```sh
cargo test --manifest-path tasks/task-051/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-051/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- Bus` produces standard output `"Arriving: Bus\n"`.
- Running `cargo run -- "Line 4"` produces standard output `"Arriving: Line 4\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-051/Cargo.toml
```
