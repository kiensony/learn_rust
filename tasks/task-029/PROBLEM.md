# task-029 — Find a shadowed value

A label printer always prints `sample`. Restore the requested label from the first argument, or an empty line when no argument was supplied. Keep the supplied argument reader.

From the repository directory, open `tasks/task-029/` and edit `src/main.rs`. The example commands below run inside that task directory. Reproduce the symptom first. Use the first diagnostic or failing example to isolate the responsible statement. Make one small repair, then rerun the same case and the acceptance tests.

```sh
cargo test --manifest-path tasks/task-029/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-029/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- red` produces standard output `"red\n"`.
- Running `cargo run -- blue` produces standard output `"blue\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-029/Cargo.toml
```
