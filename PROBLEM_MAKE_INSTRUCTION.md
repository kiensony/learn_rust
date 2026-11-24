# Authoring an exercise

Look up the learning position, module, phase, kind, and prerequisites in `course/catalog.tsv`. The original ten IDs have fixed placement at slot six of their respective modules. The stage plan and its official readings are in [course/PLAN.md](course/PLAN.md) and [course/modules.tsv](course/modules.tsv).

Read neighboring PROBLEM.md files and the existing APIs before editing. Give each exercise one teachable objective and a concrete observable result. Keep the ten legacy crates and their original student files in the root workspace. Each new exercise has its own Cargo.toml with an explicit workspace, a Rust 2024 package, and the task ID as a crate name with underscores.

## Required materials

- Write PROBLEM.md with the public contract, editable file names, examples, edge behavior, and direct Cargo commands. Give first-time programmers the vocabulary to understand the first command. Keep submission markers in the review materials.
- Supply a small editable starter. For debugging labs, reproduce one specific defect. In an explanation or refactoring task, identify the deliverable that the reviewer will inspect.
- Provide public examples and edge-case integration tests. Marked Rust submissions use a hidden equality check on `0x52540000 | N`. The legacy prefix remains `0x52535400`. A starter placeholder can be zero so the learner can run behavioral tests before adding the marker.
- Write three increasingly specific hints, one per line, in HINTS.md. For legacy tasks place the three hints in `course/legacy/task-NNN.md`. Add a concise REVIEW.md with behavior, idiom, submission, and any safety requirements.
- Use safe, dependency-free Rust through stage 5. Give later dependencies explicit versions. Label any lab requiring learner-written unsafe code or FFI and specify its toolchain, target, and safety contract.

## Verification

Work on 50 learning positions at a time. Confirm the catalog mix and prerequisites with `python3 tools/audit_course.py`. Reproduce the intended starter result, then overlay a reference in a fresh copy and run the same tests. Use structured compiler codes for diagnostic labs. Profile or test concurrency under a bounded, reproducible workload. Keep all examples local and isolate Cargo metadata and build scripts by exercise.

Use `tools/verify_pilot.py` for the first module, `tools/verify_special.py` for the Cargo, Tokio, and Miri pilots, and `tools/verify_batch.py --from-position A --through-position B` for ordinary suites. Each verifier accepts `--repairs DIR`. References live in the separate `solution` branch under `solutions/task-NNN/`.
