# task-035 — Put the package purpose in the manifest

The `--about` screen already prints Cargo's description. Set the package description in Cargo.toml to `Counts seats on the ferry`. Run the binary to inspect the result. Its single-purpose starter ignores all arguments.

From the repository directory, open `tasks/task-035/` and edit `Cargo.toml`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-035/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-035/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Counts seats on the ferry\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-035/Cargo.toml
```
