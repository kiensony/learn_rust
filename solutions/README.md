# Reference solutions

Use the task ID from the learner catalog. Open `solutions/task-011/src/main.rs` for the first example, or run its complete reference answer from this checkout with Python 3.11 or newer.

```sh
python3 solutions/check.py task-011
```

On Windows use `py -3 solutions/check.py task-011`. The runner accepts `11`, `011` and `task-011`. It copies the starter into a temporary directory, applies the reference files, and executes the task’s public and edge-case tests. Code, manifests and extra regression tests appear in a reference folder only when the answer needs them. Learner files stay in their original directories.

Use `cargo track show 11` to compare the answer with its acceptance criteria. For example, `python3 solutions/check.py 1551` exercises both feature configurations. Task 2560 uses Miri's separately pinned nightly and needs the Linux setup in its problem statement. Most answers use Rust 1.88.0. Dependency exercises may fetch their locked crates on the first run.

These directories belong to the local `solution` branch. The normal `cargo track test` command continues to test your learner starter. To open the answers beside a working course checkout, from that checkout run `git worktree add ../learn_rust-solutions solution` if the branch is available, then run the example command in the new directory. If Git reports that the branch is already checked out, `git worktree list` shows its path.

Reference libraries include the course marker so they can run the same acceptance tests as a submission. In the learner checkout, write your own implementation and follow the task’s review instructions.
