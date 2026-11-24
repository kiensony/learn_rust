# Review notes

`observe_stride_proof` returns an owned Vec<u32>. It reads the borrowed input and records stride proof in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The mutable iterator yields each visited buffer element at most once. Its step and skip operations discard positions without creating another mutable borrow.
