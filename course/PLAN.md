# Rust from first program to mastery

This course starts with a working program and one visible change. In the first session, read what the program prints, predict what editing one string will do, and compare the result with a public test. From there, alternate building a small behavior with explaining and repairing a specific failure. The [first-stage walkthrough](beginner.md) introduces the vocabulary and tools as you use them.

## Learning order

Follow the links in the stage column to begin a stage. Use `cargo track next <id>` within a stage and `cargo track list --module "Loops"` to browse a topic independently. The catalog gives each exercise a stable ID and an independent position in the recommended sequence.

| Position | Stage entry | Modules in learning order |
|---|---|---|
| 0001–0300 | [First programs](../tasks/task-011/PROBLEM.md) | Setup and Cargo · Variables and types · Numbers and conversions · Branching · Loops · Functions, strings, and first tests |
| 0301–0600 | [Ownership and borrowing](../tasks/task-309/PROBLEM.md) | Moves, Copy, and Drop · Shared borrows · Mutable borrows · Arrays, tuples, and slices · Strings and UTF-8 · Lifetime basics |
| 0601–0900 | [Modeling data and errors](../tasks/task-607/PROBLEM.md) | Structs and methods · Enums and patterns · Option and Result · Standard collections · Custom errors and ? · Modules, visibility, and rustdoc |
| 0901–1200 | [Reusable Rust](../tasks/task-903/PROBLEM.md) | Closures · Iterators · Generics and const generics · Traits and associated types · Trait objects and coherence · Lifetime-aware APIs |
| 1201–1500 | [Working with the system](../tasks/task-1201/PROBLEM.md) | Files and buffered I/O · CLIs, environment, and subprocesses · Parsing and data formats · Networking and framing · Time and portability · Integration tests |
| 1501–1800 | [Tooling and libraries](../tasks/task-1501/PROBLEM.md) | Workspaces and dependencies · Features, targets, editions, and minimum Rust versions · Test design and property tests · API compatibility and releasing crates · Declarative macros · Procedural macros and build scripts |
| 1801–2100 | [Concurrency and async](../tasks/task-1801/PROBLEM.md) | Threads and scoped threads · Send, Sync, and shared ownership · Channels and locks · Atomics and memory ordering · Futures, Pin, and wakers · Tokio, cancellation, and backpressure |
| 2101–2400 | [Algorithms and performance](../tasks/task-2101/PROBLEM.md) | Data structures and algorithms · Complexity · Allocations and zero-copy processing · CPU and memory profiling · Benchmarks and build profiles · Measured optimization |
| 2401–2700 | [Systems Rust](../tasks/task-2401/PROBLEM.md) | Layout and alignment · Unsafe contracts and raw pointers · FFI and ABI · Miri and soundness · no_std and cross-compilation · GATs, variance, and higher-ranked bounds |
| 2701–3000 | [Applied mastery](../tasks/task-2701/PROBLEM.md) | Cross-platform CLI · Reusable parsing library · Async service · WebAssembly and embedded exercises · Upgrades and incident diagnosis · Final transfer assessments |

## A 50-exercise module

| Slots | Learner activity |
|---|---|
| 1–10 | Guided practice. Read a short contract, run an example, predict a change, then edit. A returning learner can start at the first unfamiliar concept. |
| 11–30 | Applied practice. Use the same idea in another input format or calling context. Trace compiler and runtime failures back to their cause. |
| 31–45 | Mixed practice. Choose an approach, repair or refactor an earlier assumption, and explain the result. |
| 46–50 | Independent checkpoints. Start from the contract, bring a regression test and an explanation to review, and use a fresh transfer variant for a missed concept. |

Each of the 60 modules contains 25 implementations, 15 debugging labs, five refactoring or explanation tasks, and five checkpoints. Across ten stages the counts are 1,500, 900, 300, and 300 respectively. Variants revisit the same concept with different contracts, types, data, or failure modes. Aim for 5–15 minutes per beginner task and 15–30 minutes per advanced task. Record results and reassess using the [checkpoint guide](mastery.md).

The ten legacy tasks keep their student files and package names. In task-ID order they appear at positions 0256, 0156, 0706, 0306, 0556, 0606, 0656, 0756, 1056, and 0956. The other 2,990 IDs follow learning order from task-011 to task-3000.

## Running and verifying the course

The root workspace runs the local CLI and original packages. Every new task owns its workspace, so a manifest or build-script failure stays local to that task. New packages use edition 2024. Verification pins an edition-compatible stable release, Rust 1.88.0. [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) · [Rust 2024](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

The pilot consists of positions 1–50 and three later failures. Reproduce a missing workspace dependency in task-1509, drain a bounded Tokio channel in task-2090, and find a leaked allocation in task-2560. For Miri on x86-64 Linux, install nightly-2025-06-26 with the miri and rust-src components and run `cargo +nightly-2025-06-26 miri setup`. Each specialized lab names its additional commands and fixtures. [Miri setup](https://github.com/rust-lang/miri)

For a release, validate the starter and a repaired copy of each exercise. Work in 50-position batches. Match numbered diagnostics using compiler JSON and use a concrete assertion or tool finding for other failures. The audit checks ID coverage, module composition, prerequisite order, official reading links, metadata, proof ranges, and diagnostic search. Linux, macOS, and Windows CI jobs exercise the portable core. Linux jobs also cover the specialized stages. [Compiler JSON](https://doc.rust-lang.org/rustc/json.html)

## Topic and diagnostic references

The [module-to-reference map](modules.tsv) connects each module to its official language, library, or tooling reference. Join that table to [the catalog](catalog.tsv) by module name to find the individual exercises. [Rust Book](https://doc.rust-lang.org/book/) · [Rust Reference](https://doc.rust-lang.org/reference/) · [Standard library](https://doc.rust-lang.org/std/) · [Cargo Book](https://doc.rust-lang.org/cargo/)

The [diagnostic index](error-index.tsv) records the coverage and reproduction context for each published code in the audit snapshot. Search with `cargo track list E0502`. For codes covered by a supplementary compiler example, `python3 tools/error_clinic.py E0015 --prepare example.rs` extracts the indexed example from the pinned compiler and checks the code it emits. The index also identifies shared causes, retired diagnostics, and toolchain or target constraints. [Official compiler error index](https://doc.rust-lang.org/error_codes/error-index.html)

See the [verification record](VERIFICATION.md) for the commands, toolchains, and platform evidence.
