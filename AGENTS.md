# Rust Track

This repository is a sequential course. Use the learning position in `course/catalog.tsv` for difficulty and prerequisites. IDs and legacy Cargo package commands are stable.

## Review and coaching

1. Read the selected task’s REVIEW.md and PROBLEM.md. Review only the selected learner files, including any manifest or regression test the task assigns.
2. For marked Rust tasks check the first line `// rust-track: handwritten task-NNN` and `pub const TRACK_PROOF: u32`. Compare against `0x52535400 | N` for tasks 001–010 or `0x52540000 | N` for tasks 011–3000.
3. Run `cargo track test <id>` or the direct command in the task. For the original ten, `cargo test -p task_NNN` works.
4. Grade PASS or FAIL against REVIEW.md, followed by specific defects. Reject weakened acceptance tests, extra dependencies, blanket lint suppressions, incomplete implementations, and unsafe code outside a designated advanced lab. Read and evaluate every unsafe contract in a labeled lab.
5. If the student is stuck, ask what they tried, name the Rust concept, and provide one small hint. Leave implementation to the learner unless they ask for a walkthrough after attempting the problem.

## Problem authoring

When asked to create a problem or hidden tests, read [PROBLEM_MAKE_INSTRUCTION.md](PROBLEM_MAKE_INSTRUCTION.md), inspect existing titles and APIs, and use the planned module and kind. Write the starter, public and edge tests, review rubric, and progressive hints. Preserve student work. Verify a reference in a separate copy. The course checkout contains editable starters. The `solution` branch holds requested answer overlays.

## Review signals

Read unnecessary copies, unchecked unwraps, unsolicited public APIs, comments narrating individual lines, and blanket allow attributes in context. A cluster may indicate a pasted solution. A single style nit is insufficient. Keep all grading tied to the task contract and explain concrete corrections.
