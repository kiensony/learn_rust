# Course verification

The catalog covers 3,000 stable IDs in 60 modules. Each module has 25 implementations, 15 debugging labs, five refactoring or explanation tasks, and five checkpoints. The starter for a debugging lab passes verification by reproducing its documented symptom. Its reference passes by satisfying the full acceptance suite in a separate copy.

| Scope | Verification command from the course root |
|---|---|
| Structure, marker ranges, learning order, required files and 518 compiler-code records | `python3 tools/audit_course.py` |
| Cargo alias, first task, legacy navigation, filters and invalid inputs | `cargo test --locked -p track` |
| First 50 positions | `python3 tools/verify_pilot.py --jobs 4` |
| First numbered diagnostic lab through the final one | `python3 tools/verify_diagnostics.py` |
| One ordinary 50-position batch | `python3 tools/verify_batch.py --from-position 51 --through-position 100 --jobs 4` |
| Cargo manifest, Tokio shutdown and Miri pilots | `python3 tools/verify_special.py` |

For reference verification, run these Python verifiers in the `solution` branch with `--repairs solutions`. The catalog audit reads the course structure directly. The starter and repair verifiers copy the task to a temporary directory so they can build from the same acceptance tests. Use Python 3.11 or newer for the course tools.

## Release checks

All 3,000 reference answers passed the acceptance tests on Linux. The verification included the original ten packages, the first 50 learning positions, the three tool-specific pilots, 44 dedicated compiler labs, 1,103 other foundation exercises, and 1,790 other advanced exercises. The catalog records the observed failing test for each of the 2,678 ordinary assertion starters. Tests also exercised the independent checkpoint regressions in the reference overlays.

## Platform evidence

- The first 50 starter symptoms and repaired suites were exercised with Cargo on Linux. The two existing learner submissions in task-001 and task-002 already pass their tests. These results describe the files used at authoring time.
- Linux verification exercised a broken path dependency, deterministic bounded-channel shutdown, and a leak detected at the end of a Miri test process. The unsafe byte-range adapter also passed Miri on the checked inputs with nightly-2025-06-26.
- Repaired workspaces for all 1,500 portable-core exercises were cross-checked with `cargo +1.88.0 check --tests` for `x86_64-apple-darwin` and `x86_64-pc-windows-msvc`. The [CI workflow](../.github/workflows/course.yml) runs the core natively on Linux, macOS, and Windows. Hosted macOS and Windows execution is pending.
- The embedded adapter compiled for `thumbv7em-none-eabi` and the release WebAssembly library built for `wasm32-unknown-unknown`. Both release profiling workloads ran. Host permissions prevented collecting hardware performance counters, so the profiling lab was exercised using its elapsed-time fallback.
- The exercise timings are design targets. The pilot checks exercise behavior and grading. Learner completion times still need observation in a study session.
