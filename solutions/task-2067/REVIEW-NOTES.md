# Review notes

`observe_double_buffer_receiver` returns an owned Vec<u32>. It reads the borrowed input and records double buffer receiver in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
