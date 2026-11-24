# Review notes

The original search stopped at the first LF byte. Filtering and counting includes adjacent terminators. The 128 KiB fixed workload runs 1,024 scans. On this Linux host perf_event_paranoid was 4, so hardware and user-space CPU sampling were unavailable. The elapsed-time fallback measures the full workload and cannot identify hot stack frames.

Measurements in milliseconds on x86_64 Linux with Rust 1.88.0. The table reports elapsed time inside the program and excludes compilation. Other course verification jobs shared this host.

| Build | Observations (ms) | Median (ms) |
|---|---|---:|
| Starter release | 0.012, 0.012, 0.012, 0.012, 0.012 | 0.012 |
| Repair release | 110.780, 183.999, 110.703, 124.946, 111.775 | 111.775 |
