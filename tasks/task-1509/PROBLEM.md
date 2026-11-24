# task-1509 — Find the missing workspace dependency

A local library named `units` defines the clock rate for a data logger. A manifest change broke dependency resolution before the Rust compiler could start.

1. From this folder run `cargo +1.88.0 metadata --offline --format-version 1`. Read the final manifest path in Cargo's error.
2. Compare that path with the package folders on disk. Edit this folder's `Cargo.toml` to restore the inherited local dependency. Keep both packages in the workspace.
3. Run the metadata command again and identify the path source for `units`. Explain why the command failed before a single test ran.
4. Run `cargo +1.88.0 test --offline`. Two ticks must yield 30 seconds and counts that overflow the result must yield `None`.

Edit only the manifest. This lab uses the supplied local package and works offline.
