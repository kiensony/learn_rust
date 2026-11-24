# Review notes

`observe_profile_ranking` returns an owned Vec<u32>. It reads the borrowed input and records profile ranking in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

A profiler sample observes where execution was when sampling occurred. A long call can be sampled many times, and a short call can finish between samples.
