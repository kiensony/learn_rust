# Review notes

`observe_header_alignment` returns an owned Vec<u32>. It reads the borrowed input and records header alignment in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
