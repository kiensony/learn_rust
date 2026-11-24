# Review notes

The initial implementation omitted the rotation between bytes. A single-byte case leaves the accumulator rotation invisible. A two-byte sequence exposes it. The workload consumes its input and result through black_box so the optimizer cannot trivially substitute a constant result. The clock starts inside the example after compilation.

Measurements in milliseconds on x86_64 Linux with Rust 1.88.0. The table reports elapsed time inside the program and excludes compilation. Other course verification jobs shared this host.

| Build | Observations (ms) | Median (ms) |
|---|---|---:|
| Starter debug | 3784.1 | 3784.1 |
| Starter release | 47.0, 61.1, 58.0, 82.0, 65.3 | 61.1 |
| Repair debug | 5613.2 | 5613.2 |
| Repair release | 187.9, 144.3, 126.6, 148.3, 229.8 | 148.3 |
