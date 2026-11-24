# Review notes

`observe_duplicate_insertion_cost` returns an owned Vec<u32>. It reads the borrowed input and records duplicate insertion cost in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

A tree insertion costs O(log d) comparisons for d distinct keys, giving O(n log d) time and O(d) tree space. A stream introducing keys in ascending order reaches its maximum number of distinct keys.
