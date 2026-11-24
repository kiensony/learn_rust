# Review notes

`observe_embedded_checksum` returns an owned Vec<u32>. It reads the borrowed input and records embedded checksum in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The no_std dependency uses core for each byte operation. The host adapter owns the allocated vector and therefore controls its lifetime.
