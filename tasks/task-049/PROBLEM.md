# task-049 — Count the command-line values

The starter skips the executable name and counts the remaining command-line arguments. Display `arguments=` followed by this count. Quoting a value containing a space passes it as one argument.

From the repository directory, open `tasks/task-049/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-049/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-049/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- a b` produces standard output `"arguments=2\n"`.
- Running `cargo run -- "two words"` produces standard output `"arguments=1\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-049/Cargo.toml
```
