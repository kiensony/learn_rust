# Review notes

`observe_zero_copy_lines` returns an owned Vec<u32>. It reads the borrowed input and records zero copy lines in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
