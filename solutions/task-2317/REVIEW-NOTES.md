# Review notes

`observe_paired_speedup` returns an owned Vec<u32>. It reads the borrowed input and records paired speedup in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

Optimization can inline operations and remove unused work. Fixed-duration inputs give the same answer in both build profiles.
