# task-039 — Restore a missing macro argument

The summary should end with the supplied count, producing `items=3`. Repair the incomplete formatting call. Keep the number in its named binding.

From the repository directory, open `tasks/task-039/` and edit `src/main.rs`. The example commands below run inside that task directory. Reproduce the symptom first. Use the first diagnostic or failing example to isolate the responsible statement. Make one small repair, then rerun the same case and the acceptance tests.

```sh
cargo test --manifest-path tasks/task-039/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-039/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"items=3\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-039/Cargo.toml
```
