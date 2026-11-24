# task-031 — Set the package version

The program already prints the version that Cargo supplied at build time. In the package section of Cargo.toml set the version for this release to `0.2.1`. Keep the program reading its version from Cargo.

From the repository directory, open `tasks/task-031/` and edit `Cargo.toml`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-031/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-031/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"0.2.1\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-031/Cargo.toml
```
