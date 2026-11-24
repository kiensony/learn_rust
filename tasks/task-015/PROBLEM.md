# task-015 — Put a name on a ticket

The code before the printing line reads the first word passed after `cargo run --` and calls it `label`. Insert that value after `Ticket: `. An empty or missing value leaves just the prefix. Inside a format string, braces around a name display its value.

From the repository directory, open `tasks/task-015/` and edit `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-015/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-015/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- Mina` produces standard output `"Ticket: Mina\n"`.
- Running `cargo run -- Pat` produces standard output `"Ticket: Pat\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-015/Cargo.toml
```
