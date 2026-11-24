# task-040 — Share one changing status word

The program has copied the same status word into two sentences. Use the supplied library constant `STATUS` as the single source for the published word `ready`. Display `desk ready` and `line ready` on separate lines.

From the repository directory, open `tasks/task-040/` and edit `src/lib.rs`, `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-040/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-040/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"desk ready\nline ready\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-040/Cargo.toml
```
