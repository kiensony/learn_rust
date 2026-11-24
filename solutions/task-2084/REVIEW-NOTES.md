# Review notes

`observe_four_slot_intake` returns an owned Vec<u32>. It reads the borrowed input and records four slot intake in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
