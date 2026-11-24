# task-050 — Document the command in its help text

Make this program's help screen consist of two lines: `Trip log` followed by `Usage: trip <place>`. Include the angle brackets in the second line. The output stays the same for every invocation.

From the repository directory, open `tasks/task-050/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-050/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-050/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Trip log\nUsage: trip <place>\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-050/Cargo.toml
```
