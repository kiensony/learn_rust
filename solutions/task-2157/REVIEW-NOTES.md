# Review notes

`observe_linear_search_probes` returns an owned Vec<u32>. It reads the borrowed input and records linear search probes in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The observation phase can make n(n+1)/2 equality probes for n distinct bytes searched in arrival order. Its extra result storage is linear; with the fixed u8 domain, distinct keys are capped at 256.
