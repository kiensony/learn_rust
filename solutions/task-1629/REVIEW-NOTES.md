# Review notes

`observe_round_trip_property` returns an owned Vec<u32>. It reads the borrowed input and records round trip property in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
