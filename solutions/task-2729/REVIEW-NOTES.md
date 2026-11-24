# Review notes

`observe_cli_path_depth` returns an owned Vec<u32>. It reads the borrowed input and records cli path depth in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
