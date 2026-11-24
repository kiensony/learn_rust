# Review notes

`observe_prefix_comparison_budget` returns an owned Vec<u32>. It reads the borrowed input and records prefix comparison budget in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The modeled algorithm compares each new entry to its entire prefix, for n(n−1)/2 comparisons. Its count trace can be generated arithmetically in O(n) time and O(n) result space.
