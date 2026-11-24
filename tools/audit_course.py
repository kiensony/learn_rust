"""Check catalog order, module composition, exercise contents and coverage links."""

import csv
import hashlib
import re
import sys
import tomllib
from collections import Counter
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
LEGACY = {6: 1, 4: 2, 15: 3, 7: 4, 12: 5, 13: 6, 14: 7, 16: 8, 22: 9, 20: 10}
MIX = Counter(implementation=25, debugging=15, refactoring=5, checkpoint=5)
FIELDS = "id position stage module phase kind title prerequisite tags toolchain outcome requirements".split()
# SHA-256 of sorted identifiers separated by LF, from the published Rust error index.
ERROR_INDEX_DIGEST = "95bfd8fc3b5d6cb8c2def75f5cf2f15236c03dac7edd5b1b2353587ed61a227d"
READING_HOSTS = {"doc.rust-lang.org", "github.com", "tokio.rs", "docs.rs"}


def require(ok, message):
    if not ok:
        raise ValueError(message)


def table(name):
    with (ROOT / "course" / name).open(encoding="utf-8", newline="") as file:
        return list(csv.DictReader(file, delimiter="\t"))


def slug(number):
    return f"task-{number:03}"


def audit_modules(modules):
    require(len(modules) == 60, "expected 60 modules")
    require(len({row["module"] for row in modules}) == 60, "duplicate module")
    require(len({row["stage"] for row in modules}) == 10, "expected ten stages")
    for index, row in enumerate(modules):
        group = modules[(index // 6) * 6 : (index // 6 + 1) * 6]
        require(all(m["stage"] == row["stage"] for m in group), f"stage split at module {index + 1}")
        link = urlparse(row["reading"])
        require(link.scheme == "https" and link.hostname in READING_HOSTS and link.path.strip("/"),
                f"{row['module']}: missing official reading")
        if link.hostname == "github.com":
            require(link.path.startswith("/rust-lang/"), f"{row['module']}: expected Rust project source")
        require(row["tags"].strip(), f"{row['module']}: missing search terms")


def audit_index(catalog):
    errors = table("error-index.tsv")
    codes = [row["code"] for row in errors]
    require(all(re.fullmatch(r"E\d{4}", code) for code in codes), "malformed compiler code")
    digest = hashlib.sha256("\n".join(codes).encode("ascii")).hexdigest()
    require(digest == ERROR_INDEX_DIGEST, "error coverage differs from the audited published index")
    for row in errors:
        code, kind, task = row["code"], row["coverage"], row["exercise"]
        require(row["source"] == f"https://doc.rust-lang.org/error_codes/{code}.html",
                f"{code}: missing official source")
        require(bool(row["reason"].strip()), f"{code}: missing coverage reason")
        if kind in {"direct", "shared-cause"}:
            require(task in catalog, f"{code}: missing related exercise {task}")
            if kind == "direct":
                require(code in catalog[task]["diagnostics"].split(","), f"{code}: stale direct link")
        elif kind == "clinic":
            require(task == f"clinic/{code}" and int(row["sample"]) >= 0
                    and row["context"] in {"item", "body", "binary"}, f"{code}: missing compiler sample")
        else:
            require(kind in {"toolchain", "retired", "internal", "missing"} and not task,
                    f"{code}: unknown coverage kind {kind}")
    published = set(codes)
    for row in catalog.values():
        mentioned = {x for x in row["diagnostics"].split(",") if x}
        require(mentioned <= published, f"{row['id']}: unpublished diagnostic")
    return len(published)


def audit_files(row, number):
    task = row["id"]
    folder = ROOT / "tasks" / task
    for path in ("PROBLEM.md", "REVIEW.md", "Cargo.toml", "src/lib.rs", "tests/public.rs", "tests/hidden.rs"):
        require((folder / path).is_file() and (folder / path).stat().st_size > 0, f"{task}: missing {path}")
    manifest = tomllib.loads((folder / "Cargo.toml").read_text(encoding="utf-8"))
    require(manifest["package"]["name"] == task.replace("-", "_"), f"{task}: wrong package name")
    if number > 10:
        require("workspace" in manifest and manifest["package"]["edition"] == "2024",
                f"{task}: expected isolated Rust 2024 package")
    hints = ROOT / "course/legacy" / (task + ".md") if number <= 10 else folder / "HINTS.md"
    levels = [s.strip() for s in hints.read_text(encoding="utf-8").splitlines() if s.strip()]
    require(len(levels) == len(set(levels)) == 3, f"{task}: expected three progressive hints")
    if number == 1509:
        return  # Students repair a manifest here.
    proof = "0x52535400" if number <= 10 else "0x52540000"
    hidden = (folder / "tests/hidden.rs").read_text(encoding="utf-8")
    compact = re.sub(r"\s+", "", hidden)
    require("TRACK_PROOF" in compact and re.search(rf"{proof}\|{number}(?!\d)", compact) is not None,
            f"{task}: tests must check {proof} | {number}")
    problem = (folder / "PROBLEM.md").read_text(encoding="utf-8")
    require("TRACK_PROOF" not in problem and "rust-track: handwritten" not in problem,
            f"{task}: submission marker leaked into problem")


def main():
    rows = table("catalog.tsv")
    modules = table("modules.tsv")
    audit_modules(modules)
    require(len(rows) == 3000, "expected 3000 catalog records")
    by_id = {row["id"]: row for row in rows}
    expected_ids = {slug(n) for n in range(1, 3001)}
    actual_dirs = {p.name for p in (ROOT / "tasks").glob("task-*") if p.is_dir()}
    require(expected_ids == by_id.keys() == actual_dirs, "catalog and exercise folders must cover IDs 1..3000")
    versions = tomllib.loads((ROOT / "rust-toolchain.toml").read_text(encoding="utf-8"))
    stable = versions["toolchain"]["channel"]
    require(stable == "1.88.0", "stable verification dependency changed; update all tool pins together")
    nightly = tomllib.loads((ROOT / "tasks/task-2560/rust-toolchain.toml").read_text(encoding="utf-8"))
    require(nightly["toolchain"]["channel"] == "nightly-2025-06-26", "Miri needs a matching dated toolchain")
    require({"miri", "rust-src"} <= set(nightly["toolchain"]["components"]), "Miri needs miri and rust-src")
    members = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))["workspace"]["members"]
    require(set(members) == {"track", *(f"tasks/{slug(n)}" for n in range(1, 11))},
            "keep legacy tasks accessible by package and new tasks isolated")
    next_id = 11
    proofs = set()
    for position, row in enumerate(rows, 1):
        index, offset = divmod(position - 1, 50)
        mod = modules[index]
        number = LEGACY.get(index + 1) if offset == 5 else None
        if number is None:
            number, next_id = next_id, next_id + 1
        task = slug(number)
        require(all(row.get(field, "").strip() for field in FIELDS), f"{task}: missing required catalog field")
        require((row["id"], row["position"]) == (task, f"{position:04}"), f"position {position}: expected {task}")
        require((row["stage"], row["module"]) == (mod["stage"], mod["module"]), f"{task}: curriculum mismatch")
        require(row["prerequisite"] == ("none" if position == 1 else rows[position - 2]["id"]),
                f"{task}: invalid prerequisite")
        phase = "guided" if offset < 10 else "applied" if offset < 30 else "mixed" if offset < 45 else "independent"
        require(row["phase"] == phase and (row["kind"] == "checkpoint") == (offset >= 45), f"{task}: phase mismatch")
        require(row["toolchain"] == (nightly["toolchain"]["channel"] if number == 2560 else stable),
                f"{task}: unexpected toolchain")
        if offset == 49:
            require(Counter(r["kind"] for r in rows[position - 50:position]) == MIX,
                    f"{mod['module']}: exercise mix is incorrect")
        audit_files(row, number)
        if number != 1509:
            marker = (0x52535400 if number <= 10 else 0x52540000) | number
            require(marker not in proofs, f"{task}: marker collision")
            proofs.add(marker)
    count = audit_index(by_id)
    print(f"Verified 3000 tasks, 60 modules, 10 stages, {len(proofs)} distinct submission markers and {count} error-code records.")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, OSError) as error:
        sys.exit(f"Course audit failed: {error}")
