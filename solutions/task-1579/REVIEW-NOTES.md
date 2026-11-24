# Review notes

`observe_feature_packing` returns an owned Vec<u32>. It reads the borrowed input and records feature packing in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
