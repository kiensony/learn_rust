# task-011 — Run your first Rust program

Make the program display the single line `Hello, Rust!`. A program is a set of instructions. Rust begins with the instructions inside the braces of `fn main()`.

Open a terminal in the repository directory (the folder containing the course README). A terminal lets you type a command and read its result. Run this command and press Enter.

```sh
cargo run --manifest-path tasks/task-011/Cargo.toml
```

Cargo builds your Rust code and runs it. The starter prints `Hello!`. Open `tasks/task-011/src/main.rs` in your text editor. Change the message, save the file, and run the same command again. Cargo picks up the saved change.

Run the public test below. A test compares what your program did with what you expected. Try it before and after editing to see both results.

```sh
cargo test --manifest-path tasks/task-011/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-011/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Hello, Rust!\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-011/Cargo.toml
```
