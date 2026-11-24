# Review notes

`observe_quiet_option_positions` returns an owned Vec<u32>. It reads the borrowed input and records quiet option positions in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
