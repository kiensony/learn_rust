# Review notes

`observe_limited_reads` returns an owned Vec<u32>. It reads the borrowed input and records limited reads in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
