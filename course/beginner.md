# Your first 300 Rust exercises

Start with [task-011](../tasks/task-011/PROBLEM.md). The first exercise asks you to change a message on a screen. You can begin before you know what any of the punctuation means.

## Get one program running

Open a terminal in the course folder, where you can see the README and Cargo.toml. A terminal accepts commands typed on a line. Type each line below separately and press Enter. The first command opens the instructions in the terminal. The second asks Cargo, Rust’s build tool, to build and run your first program.

```sh
cargo track show task-011
cargo run --manifest-path tasks/task-011/Cargo.toml
```

Look for `Hello!` in the output. Open `tasks/task-011/src/main.rs` in a text editor. The instructions inside the braces of `fn main()` run when the program starts. The quoted text in `println!` is its message. Edit that text as the exercise asks, save the file and run the second command again.

Now run the public test, which starts the same program and compares its output with the expected message.

```sh
cargo test --manifest-path tasks/task-011/Cargo.toml --test public
```

If a test fails, look for **left** (the actual result) and **right** (the expected result). Quoted strings in a failed assertion can show a line break as `\n`. Compare punctuation, spaces and capitalization. Reopen the file to check what was saved. If it passes, follow the submission step in the task’s REVIEW.md and run the full suite. Then ask for the next exercise in learning order.

```sh
cargo track test task-011
cargo track next task-011
```

## First programs, module by module

| Start | What you will learn | Try this while you work |
|---|---|---|
| [Setup and Cargo · task-011](../tasks/task-011/PROBLEM.md) | Run, edit and test a program. Separate its visible output from build messages. Read the first diagnostic when compilation fails. | Change one word, save and rerun. Explain which file Cargo rebuilt and which text your program printed. |
| [Variables and types · task-061](../tasks/task-061/PROBLEM.md) | Give a value a name using `let`. Use numbers, booleans and characters. Change a binding using `mut`, and create a new binding using shadowing. | A receipt number of 17 should carry through to the locker number. Predict the result for a different number before running the test. |
| [Numbers and conversions · task-111](../tasks/task-111/PROBLEM.md) | Use signed numbers for measurements below zero, widen numbers before arithmetic, and keep fractions with floating point. | A thermometer rises from −2 to 3. Write the two operands and expected change on paper before coding. |
| [Branching · task-161](../tasks/task-161/PROBLEM.md) | Choose among outcomes using `if`, `else` and `match`. Combine tests and handle values at a boundary. | Place one example on each side of a threshold and a third exactly on the threshold. Decide which branch handles each one. |
| [Loops · task-210](../tasks/task-210/PROBLEM.md) | Repeat work with `for` and `while`. Use a counter or running total, stop early with `break` and skip an iteration with `continue`. | Trace the current input, running value and next action for three iterations. Try an input that requires zero iterations. |
| [Functions, strings, and first tests · task-260](../tasks/task-260/PROBLEM.md) | Give a computation a name, pass input, return a result, inspect borrowed text and write your own unit test. | Write down the function’s input and output types. Predict one ordinary return value and one boundary value. |

## Read a function signature

For example, `fn is_quiet(volume: u8) -> bool` names a function `is_quiet`. Its input is named `volume`. The type `u8` means a whole number from 0 through 255. The type after the arrow says what the caller will receive. Here, `bool` means `true` or `false`. Calling the function looks like `is_quiet(2)`.

A semicolon finishes a statement. At the end of a function, an expression without a trailing semicolon supplies the return value. If a compiler message says it found `()`, check whether your final expression ended in a semicolon. The empty pair of parentheses represents Rust’s unit value.

## Get unstuck

1. Reproduce the symptom by running the command from PROBLEM.md on the starter. Write down the input, the expected result and what actually happened.
2. If the program fails to compile, read the first error and its marked source line. If a test fails after compilation, reduce its input until you can trace the wrong value by hand.
3. Change one thing that explains that observation. Run the same case again, then run the rest of the task’s tests.
4. If you need a nudge, use `cargo track hint <id> 1`. Levels 2 and 3 become more specific. For coaching, share your prediction, your edit and the exact error or failed assertion.

Choose the next exercise by its learning position. The ten original tasks keep their historical IDs, so the number sometimes jumps. The last five exercises in each module are independent checkpoints. Use the [checkpoint guide](mastery.md) to record a pass or select a fresh practice case.
