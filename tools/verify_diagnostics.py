"""Check diagnostic starters with rustc JSON, and optionally test external repairs."""

import argparse
from concurrent.futures import ThreadPoolExecutor
import csv
import json
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
TOOLCHAIN = "+1.88.0"


def verify(row, tasks, repairs):
    task = f"task-{int(row['id']):03}"
    with tempfile.TemporaryDirectory(prefix="rust-diagnostic-") as work:
        folder = Path(work) / task
        shutil.copytree(tasks / task, folder, ignore=shutil.ignore_patterns("target"))
        run = subprocess.run(
            ["cargo", TOOLCHAIN, "check", "--offline", "--message-format=json"],
            cwd=folder, capture_output=True, text=True, timeout=60,
        )
        messages = (json.loads(line) for line in run.stdout.splitlines())
        errors = {
            item["message"]["code"]["code"]
            for item in messages if item.get("reason") == "compiler-message"
            and item["message"]["level"] == "error" and item["message"].get("code")
        }
        if run.returncode == 0 or row["diagnostics"] not in errors:
            raise ValueError(f"{task}: expected {row['diagnostics']}, found {errors}\n{run.stdout}")
        if repairs:
            shutil.copyfile(repairs / task / "src/lib.rs", folder / "src/lib.rs")
            test = subprocess.run(
                ["cargo", TOOLCHAIN, "test", "--offline", "--quiet"],
                cwd=folder, capture_output=True, text=True, timeout=60,
            )
            if test.returncode:
                raise ValueError(f"{task}: repair failed\n{test.stderr}{test.stdout}")
    return task


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tasks", type=Path, default=ROOT / "tasks")
    parser.add_argument("--repairs", type=Path)
    args = parser.parse_args()
    with (ROOT / "course/diagnostics.tsv").open() as table:
        rows = list(csv.DictReader(table, delimiter="\t"))
    with ThreadPoolExecutor(max_workers=3) as workers:
        futures = [workers.submit(verify, row, args.tasks, args.repairs) for row in rows]
        failures = []
        for future in futures:
            try:
                future.result()
            except (OSError, ValueError, subprocess.SubprocessError) as error:
                failures.append(str(error))
    if failures:
        raise SystemExit("\n".join(failures))
    modes = "starters and repairs" if args.repairs else "starters"
    print(f"Verified {len(rows)} diagnostic {modes} on Rust {TOOLCHAIN[1:]}.")


if __name__ == "__main__":
    main()
