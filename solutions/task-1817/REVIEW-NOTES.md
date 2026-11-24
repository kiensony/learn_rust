# Review notes

`observe_shard_maxima` returns an owned Vec<u32>. It reads the borrowed input and records shard maxima in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The scoped worker borrows slices until its handle joins. Joining and concatenating the two shard ranges in index order makes the result deterministic.
