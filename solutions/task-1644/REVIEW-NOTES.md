# Review notes

`observe_shift_property` returns an owned Vec<u32>. It reads the borrowed input and records shift property in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
