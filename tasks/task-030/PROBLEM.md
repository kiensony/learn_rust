# task-030 — Extract a reusable banner

The kiosk's heading decoration must be reusable. Complete the supplied library function `pub fn banner(label: &str) -> String` to produce `== label ==`. Make the binary call it for its first argument. Empty input gives `==  ==`. In Rust, the final expression in a function gives its return value.

From the repository directory, open `tasks/task-030/` and edit `src/lib.rs`, `src/main.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-030/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-030/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- Kitchen` produces standard output `"== Kitchen ==\n"`.
- Running `cargo run -- Hall` produces standard output `"== Hall ==\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-030/Cargo.toml
```
