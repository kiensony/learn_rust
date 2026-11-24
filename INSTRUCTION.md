# Learning with Rust Track

Begin at task-011. Run the example first. Write down what you think one small edit will change, save the edit, and run it again. A compiler error is feedback about a rule Rust checked. A failing assertion compares what your program returned with what the exercise asked for.

## A practice session

1. Read `cargo track show <id>`. Each new task has its own small Cargo project. The task tells you which source or manifest to edit.
2. Try one public example on paper. Run the example or the public test. The first tasks take about 5–15 minutes. Allow 15–30 minutes for an advanced task, plus time for any initial tool installation.
3. Use `cargo track check <id>` after an edit. Test behavior with the task’s public suite. Start with hint 1 when you get stuck, then try your own idea before opening another hint.
4. Finish the submission step below, run `cargo track test <id>`, and use REVIEW.md for a short review. Keep the provided acceptance tests intact. Put your own regression in a new test file when the exercise asks for one.
5. Use `cargo track next <id>` to continue in learning order. You can search for any topic and join a later module at its first guided task.

## Debugging diary

Reproduce the symptom using the command in PROBLEM.md. Record the first failing test or compiler code. Reduce the example until one input, type, configuration value, or scheduling event explains the failure. Predict the result of a targeted edit. Run the original reproduction again, then the complete suite. Record one input that would catch the bug if it returned.

For a numbered error, try `rustc --explain E0502` and `cargo track list E0502`. The compiler clinic can also prepare a small example for many errors. Use `python3 tools/error_clinic.py E0015 --prepare example.rs` from the repository root.

## Submission marker

For a Rust task with a marker test, make the first line of the learner’s library `// rust-track: handwritten task-NNN` using the actual folder name. Add or update the public constant in that library. The prefix depends on the stable task ID.

| Task | Constant example |
|---|---|
| Original tasks 001–010 | `pub const TRACK_PROOF: u32 = 0x52535400 \| 1;` for task-001 |
| New marked tasks 011–3000 | `pub const TRACK_PROOF: u32 = 0x52540000 \| 11;` for task-011 |

Use the decimal task number on the right. In a binary exercise, REVIEW.md points to the library where the marker belongs. Manifest-only labs can be submitted using the files named in their rubric.

## Reviewing work

Use safe Rust and the supplied standard library through stage 5. Later stages name any provided dependency in the exercise manifest. Learner-written unsafe operations and foreign interfaces belong only in explicitly labeled advanced labs. Review their safety argument alongside the tests.

Ask a coach for a hint by giving the task ID, the command you ran, one input, the symptom, and what you tried. At a checkpoint, use documentation freely, write your own regression, and explain a design tradeoff. Aim to pass four of five independent checkpoints in each module and every safety-critical checkpoint. See the [mastery guide](course/mastery.md).

Authors use [the problem authoring guide](PROBLEM_MAKE_INSTRUCTION.md).
