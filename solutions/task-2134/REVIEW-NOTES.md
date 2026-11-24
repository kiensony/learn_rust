# Review notes

`observe_histogram_bins` returns an owned Vec<u32>. It reads the borrowed input and records histogram bins in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The data structure handles ordering and membership while the caller independently computes the requested report.
