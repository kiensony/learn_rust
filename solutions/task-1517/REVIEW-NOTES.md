# Review notes

`observe_packet_low_nibble` returns an owned Vec<u32>. It reads the borrowed input and records packet low nibble in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The dependency key wire is the Rust import name. The package field selects track-wire in the wire directory.
