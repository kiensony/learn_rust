# Review notes

`observe_unique_owner_state` returns an owned Vec<u32>. It reads the borrowed input and records unique owner state in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

Cloning an Arc shares ownership of the same allocation. The scoped owner and observer handles drop before the next input byte is processed.
