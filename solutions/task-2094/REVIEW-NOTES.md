# Review notes

`observe_five_slot_ingress` returns an owned Vec<u32>. It reads the borrowed input and records five slot ingress in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
