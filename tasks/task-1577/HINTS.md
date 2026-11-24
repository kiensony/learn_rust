Run cargo test --test link_probe -- --nocapture from the package directory.
Read the compiler invocation and find the -lstatic archive selection.
Compare the name produced by rustc --crate-name with the name passed to the link step.
