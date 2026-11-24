# Review notes

`observe_sorted_insertion_moves` returns an owned Vec<u32>. It reads the borrowed input and records sorted insertion moves in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

Stable insertion can shift n(n−1)/2 entries for descending distinct values. The sorted vector uses linear space. Even with two distinct byte keys, inserting smaller late arrivals can cause quadratic total movement.
