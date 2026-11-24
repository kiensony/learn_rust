# Review notes

`observe_first_frame_byte` returns an owned Vec<u32>. It reads the borrowed input and records first frame byte in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
