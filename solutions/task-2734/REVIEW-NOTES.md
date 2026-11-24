# Review notes

`observe_cli_rust_sources` returns an owned Vec<u32>. It reads the borrowed input and records cli rust sources in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
