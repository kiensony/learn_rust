"""Run a reference answer with the course's acceptance tests in a temporary crate."""

import argparse
import csv
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib

SOLUTIONS = Path(__file__).resolve().parent
COURSE = SOLUTIONS.parent


def task_id(text):
    if not re.fullmatch(r"(?:task-)?[0-9]{1,4}", text):
        raise argparse.ArgumentTypeError("use a task ID such as task-011 or 11")
    number = int(text.removeprefix("task-"))
    if not 1 <= number <= 3000:
        raise argparse.ArgumentTypeError("task number must be in 1..3000")
    return f"task-{number:03}"


def run(task):
    with (COURSE / "course/catalog.tsv").open(encoding="utf-8", newline="") as table:
        record = next((r for r in csv.DictReader(table, delimiter="\t") if r["id"] == task), None)
    if not record:
        raise ValueError(f"{task} is missing from the catalog")
    reference = SOLUTIONS / task
    if not reference.is_dir():
        raise ValueError(f"missing reference directory {reference}")

    with tempfile.TemporaryDirectory(prefix=f"rust-solution-{task}-") as tmp:
        crate = Path(tmp) / task
        shutil.copytree(COURSE / "tasks" / task, crate, ignore=shutil.ignore_patterns("target"))
        for file in sorted(reference.rglob("*")):
            if not file.is_file():
                continue
            relative = file.relative_to(reference)
            dest = crate / relative
            editable_test = task == "task-060" and relative == Path("tests/regression.rs")
            if relative.parts[0] == "tests" and dest.exists() and not editable_test:
                raise ValueError(f"reference would overwrite a course test: {relative}")
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(file, dest)
        manifest = crate / "Cargo.toml"
        if int(task[5:]) <= 10:
            with manifest.open("a", encoding="utf-8") as out:
                out.write("\n[workspace]\n")
        pin = record["toolchain"]
        cargo = ["cargo", f"+{pin}"]
        if task == "task-2560":
            cargo.append("miri")
        cargo.append("test")
        if (crate / "Cargo.lock").is_file():
            cargo.append("--locked")
        env = {**os.environ, "CARGO_TARGET_DIR": str(Path(tmp) / "target")}
        commands = [cargo]
        if "wide" in tomllib.loads(manifest.read_text(encoding="utf-8")).get("features", {}):
            commands.append([*cargo, "--features", "wide"])
        for command in commands:
            print(f"{task}  {' '.join(command)}", flush=True)
            result = subprocess.run(command, cwd=crate, env=env, timeout=240)
            if result.returncode:
                return result.returncode
    return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("task", type=task_id)
    try:
        sys.exit(run(parser.parse_args().task))
    except (OSError, ValueError, subprocess.TimeoutExpired) as error:
        sys.exit(f"solution: {error}")
