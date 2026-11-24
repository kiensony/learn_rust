# Review notes

`observe_major_version` returns an owned Vec<u32>. It reads the borrowed input and records major version in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

Removing this function or changing its return type breaks downstream callers and would require a major version change for a stable release.
