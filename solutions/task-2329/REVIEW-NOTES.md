# Review notes

`observe_batch_medians` returns an owned Vec<u32>. It reads the borrowed input and records batch medians in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

Optimization can inline operations and remove unused work. Fixed-duration inputs give the same answer in both build profiles.
