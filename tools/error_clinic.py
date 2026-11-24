"""Open an indexed compiler exercise or regenerate coverage against the course pin."""

import argparse
import concurrent.futures
import csv
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
INDEX = ROOT / "course/error-index.tsv"
PIN = "+1.88.0"
EDITION = "2024"
COLUMNS = ["code", "coverage", "exercise", "sample", "context", "reason", "source"]


def run_rustc(args, **kwargs):
    return subprocess.run(
        ["rustc", PIN, *args], encoding="utf-8", errors="replace",
        capture_output=True, timeout=30, **kwargs,
    )


def explanation(code):
    run = run_rustc(["--explain", code], env={**os.environ, "PAGER": "cat"})
    return run.stdout if run.returncode == 0 else ""


def samples(doc):
    return re.findall(r"^```[^\n]*\n(.*?)^```[ \t]*$", doc, re.M | re.S)


def source(sample, context):
    if context in {"item", "binary"}:
        return sample
    return "fn main() {\n" + sample + "\n}\n"


def diagnostic_codes(path, context):
    result = run_rustc(
        ["--crate-name", "clinic", "--crate-type", "bin" if context == "binary" else "lib", "--emit=metadata",
         "--edition", EDITION, "--error-format=json", str(path),
         "-o", str(path.with_suffix(".rmeta"))]
    )
    messages = [json.loads(line) for line in result.stderr.splitlines() if line.startswith("{")]
    return sorted({
        m["code"]["code"] for m in messages if m.get("level") == "error" and m.get("code")
    })


def probe(code):
    """Compile the pin's reference snippets without executing example programs."""
    doc = explanation(code)
    if not doc:
        return ("missing", -1, "-", "This published code is absent from rustc 1.88.0 --explain.")
    if re.search("no longer emitted|no-longer emitted", doc[:180], re.I):
        return ("retired", -1, "-", "The pinned compiler explanation marks this diagnostic as retired.")
    if "internal to the compiler" in doc[:180]:
        return ("internal", -1, "-", "The pinned explanation identifies an internal diagnostic.")
    observed = set()
    with tempfile.TemporaryDirectory(prefix="rust-clinic-") as tmp:
        path = Path(tmp) / "example.rs"
        for index, sample in enumerate(samples(doc)):
            contexts = ["item", "body"]
            if "fn main" in sample:
                contexts.append("binary")
            for context in contexts:
                path.write_text(source(sample, context), encoding="utf-8")
                codes = diagnostic_codes(path, context)
                if code in codes:
                    return ("clinic", index, context, "The pinned compiler emits this code on the indexed example.")
                observed.update(codes)
    others = ", ".join(sorted(observed))
    outcome = f"observed {others}" if others else "observed no numbered error"
    return ("review", -1, "-", f"Edition 2024 on x86_64-unknown-linux-gnu {outcome} in the published snippets.")


def read_table(path):
    with path.open(encoding="utf-8", newline="") as file:
        return list(csv.DictReader(file, delimiter="\t"))


def compile_index(codes):
    overrides = {r["code"]: r for r in read_table(ROOT / "course/error-related.tsv")}
    direct = {r["diagnostics"]: int(r["id"]) for r in read_table(ROOT / "course/diagnostics.tsv")}
    direct.update({r["diagnostics"]: int(r["id"]) for r in read_table(ROOT / "course/pilot.tsv")
                   if re.fullmatch(r"E\d{4}", r["diagnostics"])})
    for row in read_table(ROOT / "course/catalog.tsv"):
        if row["kind"] == "debugging":
            for code in row["diagnostics"].split(","):
                if re.fullmatch(r"E[0-9]{4}", code):
                    direct.setdefault(code, int(row["id"][5:]))
    results = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        for code, (kind, index, context, note) in zip(codes, pool.map(probe, codes)):
            task = ""
            if code in direct:
                kind = "direct"
                task = f"task-{direct[code]:03}"
                note = "The starter's compiler output contains this diagnostic."
            elif kind == "clinic":
                task = f"clinic/{code}"
            elif kind == "review" and code in overrides:
                kind = "shared-cause" if overrides[code]["exercise"] else "toolchain"
                task = overrides[code]["exercise"]
                note = overrides[code]["reason"]
            results.append(dict(zip(COLUMNS, [code, kind, task, index, context, note,
                f"https://doc.rust-lang.org/error_codes/{code}.html"])))
    return results


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("code", nargs="?", help="for example E0502")
    parser.add_argument("--prepare", type=Path, help="create an editable copy at a new .rs path")
    parser.add_argument("--audit", type=Path, metavar="INDEX_HTML",
                        help="reprobe every published code in a local copy of the official error index")
    args = parser.parse_args()
    if args.audit:
        index = args.audit.read_text(encoding="utf-8")
        codes = sorted(set(re.findall(r"\bE\d{4}\b", index)))
        if len(codes) < 500:
            parser.error("expected the full official error index HTML")
        with tempfile.NamedTemporaryFile(mode="w", encoding="utf-8", dir=INDEX.parent,
                                         prefix="error-index-", delete=False) as file:
            writer = csv.DictWriter(file, COLUMNS, delimiter="\t", lineterminator="\n")
            writer.writeheader()
            writer.writerows(compile_index(codes))
        os.chmod(file.name, 0o644)
        os.replace(file.name, INDEX)
        print(f"Audited {len(codes)} published codes on the pinned compiler.")
        return
    if not args.code or not re.fullmatch(r"[Ee][0-9]{4}", args.code):
        parser.error("supply an error code such as E0502")
    code = args.code.upper()
    row = next((r for r in read_table(INDEX) if r["code"] == code), None)
    if not row:
        parser.error(f"{code} is absent from the published index")
    print(f"{code} ({row['coverage']}) {row['exercise']}\n{row['reason']}\n{row['source']}")
    if row["exercise"].startswith("task-"):
        print("cargo track show", row["exercise"])
    if row["sample"] == "-1":
        if args.prepare:
            parser.error("this entry has no reproducible single-file sample on the course pin")
        return
    if args.prepare is None:
        print(f"python3 tools/error_clinic.py {code} --prepare example.rs")
        return
    path = args.prepare
    if path.suffix != ".rs":
        parser.error("choose a new .rs file for the editable example")
    blocks = samples(explanation(code))
    content = source(blocks[int(row["sample"])], row["context"])
    # Check an untouched sample before reserving the user's file.
    with tempfile.TemporaryDirectory(prefix="rust-clinic-") as tmp:
        sample = Path(tmp) / "sample.rs"
        sample.write_text(content, encoding="utf-8")
        if code not in diagnostic_codes(sample, row["context"]):
            parser.error(f"the installed pin no longer reproduces {code} for this host")
    try:
        with path.open("x", encoding="utf-8") as file:
            file.write(content)
    except FileExistsError:
        parser.error(f"{path} already exists; select an unused path")
    crate_type = "bin" if row["context"] == "binary" else "lib"
    print(f'Reproduce {code}, then repair the file and rerun the same command:\n'
          f'rustc {PIN} --crate-name clinic --crate-type {crate_type} --emit metadata --edition {EDITION} "{path}"')


if __name__ == "__main__":
    main()
