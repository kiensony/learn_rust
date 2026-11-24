Read the last “failed to read” path. Which package did Cargo try to open?
Look at both workspace.members and workspace.dependencies in Cargo.toml.
An inherited path is relative to the workspace root. Compare its spelling with the folder that contains the units manifest.
