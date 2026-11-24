# Review notes

`observe_flag_fields` returns an owned Vec<u32>. It reads the borrowed input and records flag fields in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
