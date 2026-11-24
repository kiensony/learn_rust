# task-037 — Fix an accidental extra terminator

A supplied receipt template ends with a newline already. Display every byte of `receipt.txt` exactly once. An extra newline leaves an unwanted blank line on the printer.

From the repository directory, open `tasks/task-037/` and edit `src/main.rs`. The example commands below run inside that task directory. Reproduce the symptom first. Use the first diagnostic or failing example to isolate the responsible statement. Make one small repair, then rerun the same case and the acceptance tests.

```sh
cargo test --manifest-path tasks/task-037/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-037/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"PAID\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-037/Cargo.toml
```
