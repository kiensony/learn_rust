# Review notes

`observe_borrowed_prefixes` returns an owned Vec<u32>. It reads the borrowed input and records borrowed prefixes in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
