"""Exercise the first module on pinned Rust in disposable copies.

Pass --repairs DIR to test independent author repairs at DIR/task-NNN using
the original acceptance tests. Without that directory check starter symptoms.
"""

import argparse
import csv
import json
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def cargo_test(directory, public_only=False):
    command = ["cargo", "+1.88.0", "test", "--offline", "--message-format=json"]
    if public_only:
        command.extend(["--test", "public"])
    return subprocess.run(
        command, cwd=directory, capture_output=True,
        encoding="utf-8", errors="replace", timeout=120,
    )


def observed_failure(record, result):
    if result.returncode == 0:
        return False
    output = result.stdout + result.stderr
    messages = []
    for line in result.stdout.splitlines():
        if line.startswith("{"):
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue
            if isinstance(event, dict) and event.get("reason") == "compiler-message":
                messages.append(event["message"])
    code = record["diagnostics"]
    if code:
        return any((m["code"] or {}).get("code") == code for m in messages)
    category, detail = record["outcome"].split(": ", 1)
    if category == "compiler":
        return any(detail in m["message"] for m in messages if m["level"] == "error")
    if category == "runtime":
        return "program failed:" in output and "panicked" in output
    symptom = {
        "reusable library item": ("reusable_label ... FAILED", "reusable_value ... FAILED"),
        "preview example output": ("selected_target ... FAILED",),
        "default binary output": ("selected_target ... FAILED",),
        "shared prompt file": ("shared_resource ... FAILED",),
        "output streams": ("standard output", "standard error"),
        "stdout": ("standard output",),
    }[detail]
    return "test result: FAILED" in output and any(s in output for s in symptom)


def verify(record, repairs):
    task = f"task-{int(record['id']):03d}"
    with tempfile.TemporaryDirectory(prefix=f"{task}-") as temp:
        directory = Path(temp) / task
        shutil.copytree(ROOT / "tasks" / task, directory, ignore=shutil.ignore_patterns("target"))
        result = cargo_test(directory, public_only=True)
        if not observed_failure(record, result):
            raise RuntimeError(f"{task}: unexpected starter result\n{result.stdout}\n{result.stderr}")
        if repairs is not None:
            repaired = repairs / task
            for source in repaired.rglob("*"):
                relative = source.relative_to(repaired)
                if relative.parts[0] in {"tests", "target"} or not source.is_file():
                    continue
                dest = directory / relative
                dest.parent.mkdir(parents=True, exist_ok=True)
                # New timestamps cause Cargo to rebuild changed files.
                dest.write_bytes(source.read_bytes())
            # The final checkpoint asks the learner to add their own regression.
            if task == "task-060":
                shutil.copyfile(repaired / "tests/regression.rs", directory / "tests/regression.rs")
            result = cargo_test(directory)
            if result.returncode:
                raise RuntimeError(f"{task}: repair failed\n{result.stdout}\n{result.stderr}")
    return task


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repairs", type=Path)
    parser.add_argument("--jobs", type=int, default=2)
    args = parser.parse_args()
    with (ROOT / "course/pilot.tsv").open(encoding="utf8", newline="") as f:
        records = list(csv.DictReader(f, delimiter="\t"))
    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for task in pool.map(lambda record: verify(record, args.repairs), records):
            print(task, "starter and repair OK" if args.repairs else "starter OK", flush=True)
