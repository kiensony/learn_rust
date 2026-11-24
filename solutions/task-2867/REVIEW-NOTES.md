# Review notes

`observe_big_endian_registers` returns an owned Vec<u32>. It reads the borrowed input and records big endian registers in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The two byte positions have a specified order in the register format. The decoder interprets those positions explicitly on every host.
