# task-048 — Select a license tag

The maintainer chose `MIT` as the package license. Set its Cargo manifest license field so this supplied program prints `License: MIT`. The program must continue to pick up future changes to that field.

From the repository directory, open `tasks/task-048/` and edit `Cargo.toml`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-048/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-048/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"License: MIT\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-048/Cargo.toml
```
