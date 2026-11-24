# task-044 — Read a supplied environment setting

The starter reads the environment variable `TRACK_ROOM` into the name `room`. Print `Room: ` followed by the setting, ending with one newline. When the setting is empty or missing, print the bare prefix and its space.

From the repository directory, open `tasks/task-044/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-044/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-044/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- With TRACK_ROOM set to `West`, running `cargo run` produces standard output `"Room: West\n"`.
- With TRACK_ROOM set to `Blue`, running `cargo run` produces standard output `"Room: Blue\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-044/Cargo.toml
```
