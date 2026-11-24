# task-012 — Give a notice two lines

A station sign needs two lines in order: `North gate` and `Open`. Each call to `println!` finishes one line. Add the second instruction inside the braces.

From the repository directory, open `tasks/task-012/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-012/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-012/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"North gate\nOpen\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-012/Cargo.toml
```
