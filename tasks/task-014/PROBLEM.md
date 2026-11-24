# task-014 — Leave a blank line on a postcard

Print `Dear Jo,`, a completely empty line, and `See you soon.`. An empty call to `println!()` prints just a line break.

From the repository directory, open `tasks/task-014/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-014/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-014/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Dear Jo,\n\nSee you soon.\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-014/Cargo.toml
```
