# Practice a compiler diagnostic

From the course root, run `cargo track list E0502` to find the vector-borrow exercise. For any code in the published index, look up the course exercise, a supplemental compiler example, or the reason it depends on a different toolchain or target.

```sh
python3 tools/error_clinic.py E0502
python3 tools/error_clinic.py E0015 --prepare clock.rs
rustc +1.88.0 --crate-name clinic --crate-type lib --emit metadata --edition 2024 clock.rs
```

Use the compiler command printed for your particular example. It checks a single standalone source file. Start with the primary error span and underline the two declarations or expressions involved. Read `rustc +1.88.0 --explain E0015`, then change the smallest piece of the example that addresses the cause. Keep the original operations that demonstrated the problem and repeat the command. Explain which Rust rule now holds. A task linked from the index also has behavioral acceptance tests.

`--prepare` verifies the diagnostic on your host before creating a file. It takes an unused filename, so each new practice attempt starts separately from earlier work. Some reference examples touch advanced Rust features. Their compiler command stops after metadata generation.

## How to read the coverage table

Each row in `error-index.tsv` links to its official explanation. A **direct** row names a numbered course task verified to emit the code. A **clinic** row gives an additional exercise extracted from the installed Rust 1.88.0 explanation and reproduced using structured rustc diagnostics. A **shared-cause** row points to a related numbered task with an explanation of the connection. Other rows record toolchain, target, retirement, or compiler-internal status. The index includes codes published after the pinned compiler.

To refresh coverage on x86-64 Linux, download the official error-index HTML and run the audit. Use `tools/verify_diagnostics.py` and `tools/verify_pilot.py` to cross-check the numbered starters. Review toolchain explanations and new codes when updating the catalog.

```sh
curl -fsSLo /tmp/rust-error-index.html https://doc.rust-lang.org/error_codes/error-index.html
python3 tools/error_clinic.py --audit /tmp/rust-error-index.html
```

The example text prepared from `rustc --explain` is part of the Rust project. [Error index](https://doc.rust-lang.org/error_codes/error-index.html) · [Rust project licensing](https://github.com/rust-lang/rust/blob/1.88.0/COPYRIGHT)
