# task-055 — Replace scattered number formatting

Move receipt numbering into `pub fn receipt(number: &str) -> String`. It must return `R-` followed by the supplied number exactly, including leading zeroes. Use the function to print the first argument; missing input produces `R-`.

From the repository directory, open `tasks/task-055/` and edit `src/main.rs`, `src/lib.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-055/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-055/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- 101` produces standard output `"R-101\n"`.
- Running `cargo run -- 007` produces standard output `"R-007\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-055/Cargo.toml
```
