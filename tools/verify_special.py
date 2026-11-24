"""Check specialized pilot starters and, optionally, external repaired copies."""

import argparse
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
STABLE = "+1.88.0"
MIRI = "+nightly-2025-06-26"


def check(folder, command, expected_failure=None):
    run = subprocess.run(
        command, cwd=folder, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        encoding="utf-8", errors="replace", timeout=180,
    )
    if expected_failure is None:
        ok = run.returncode == 0
    else:
        ok = run.returncode != 0 and all(text in run.stdout for text in expected_failure)
    if not ok:
        raise SystemExit(f"{folder.name} {command}\n{run.stdout}")
    print(f"{folder.name}: {'expected failure' if expected_failure else 'pass'} ({' '.join(command)})")


def verify(repair_root):
    with tempfile.TemporaryDirectory(prefix="rust-pilots-") as work:
        for number in (1509, 2090, 2560):
            name = f"task-{number}"
            tmp = Path(work) / name
            shutil.copytree(ROOT / "tasks" / name, tmp, ignore=shutil.ignore_patterns("target"))
            if number == 1509:
                check(tmp, ["cargo", STABLE, "metadata", "--offline", "--format-version", "1"],
                      ["failed to load manifest for dependency", "unit", "Cargo.toml"])
                suite = ["cargo", STABLE, "test", "--offline"]
            elif number == 2090:
                check(tmp, ["cargo", STABLE, "test", "--locked", "--test", "public"],
                      ["shutdown_drains_the_whole_queue ... FAILED", "left: [7]", "right: [7, 2, 7]"])
                suite = ["cargo", STABLE, "test", "--locked"]
            else:
                check(tmp, ["cargo", STABLE, "test", "--offline"])
                check(tmp, ["cargo", MIRI, "miri", "test", "--test", "public", "acknowledge_one_sample"],
                      ["memory leaked"])
                suite = ["cargo", MIRI, "miri", "test"]
            if repair_root:
                shutil.copytree(repair_root / name, tmp, dirs_exist_ok=True, ignore=shutil.ignore_patterns("target"))
                # Cargo may have built the starter in this same directory. A repair
                # overlay can retain older mtimes, so force a clean rebuild.
                shutil.rmtree(tmp / "target", ignore_errors=True)
                check(tmp, suite)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repairs", type=Path, help="external directory of repaired task-NNN files")
    verify(parser.parse_args().repairs)
