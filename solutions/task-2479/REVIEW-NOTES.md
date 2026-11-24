# Review notes

`observe_strided_raw_reads` returns an owned Vec<u32>. It reads the borrowed input and records strided raw reads in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The input slice owns no storage but borrows a live allocation. Each visited index is in bounds for a byte read. A pointer immediately past the slice may mark the end of a range but points to no readable element of the slice.
