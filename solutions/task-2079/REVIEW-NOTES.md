# Review notes

`observe_three_slot_mailbox` returns an owned Vec<u32>. It reads the borrowed input and records three slot mailbox in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
