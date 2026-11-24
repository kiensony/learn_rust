"""Verify starter symptoms and test external answer overlays in disposable workspaces."""
import argparse
import concurrent.futures
import csv
import json
import os
import re
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]

def suite(folder, command, target, timeout):
    env = dict(os.environ, CARGO_TARGET_DIR=str(target), CARGO_INCREMENTAL="0",
               CARGO_PROFILE_TEST_DEBUG="0", CARGO_TERM_COLOR="never")
    run = subprocess.run(command, cwd=folder, env=env, capture_output=True,
                         encoding="utf-8", errors="replace", timeout=timeout)
    return run.returncode, run.stdout + run.stderr

def observed(row, status, output):
    if not status:
        return row['outcome'] == 'starter passes'
    codes = set()
    for line in output.splitlines():
        if not line.startswith('{'):
            continue
        try:
            msg = json.loads(line).get('message', {})
            if isinstance(msg, dict) and msg.get('level') == 'error' and msg.get('code'):
                codes.add(msg['code']['code'])
        except ValueError:
            continue
    if row['diagnostics']:
        return bool(set(row['diagnostics'].split(',')) & codes)
    named = re.fullmatch(r'assertion: (public_examples|edge_cases)(?: \(expected (.*), got (.*)\))?', row['outcome'])
    if named:
        test, expected, actual = named.groups()
        if f'test {test} ... FAILED' not in output:
            return False
        if expected is None:
            return True
        details = output.partition(f'---- {test} stdout ----')[2].partition('\nfailures:')[0]
        return f'left: {actual}\n right: {expected}' in details
    if row['outcome'] == 'assertion: public_examples or edge_cases':
        return any(f'test {name} ... FAILED' in output for name in ('public_examples', 'edge_cases'))
    # A missing proof is separately checked by review. Demand a behavioral failure.
    return any(' ... FAILED' in line and 'submission_marker' not in line and
               'proof' not in line for line in output.splitlines())

def verify(row, repairs, offline=False, timeout=120):
    name = row['id']
    with tempfile.TemporaryDirectory(prefix='track-check-') as temp:
        base = Path(temp)
        folder = base / name
        src = ROOT / 'tasks' / name
        shutil.copytree(src, folder, ignore=shutil.ignore_patterns('target'))
        if int(name[5:]) <= 10:
            with (folder / 'Cargo.toml').open('a') as f:
                f.write('\n[workspace]\n')
        cargo = ['cargo', '+1.88.0', 'test', '--no-fail-fast', '--message-format=json']
        if offline:
            cargo.append('--offline')
        red, log = suite(folder, cargo, base/'target', timeout)
        if not observed(row, red, log):
            raise ValueError('unexpected starter outcome\n' + log[-8000:])
        if repairs is None:
            return
        answer = repairs / name
        if not answer.is_dir():
            raise ValueError('missing reference answer')
        for source in (answer / 'tests').rglob('*'):
            if source.is_file() and (folder / source.relative_to(answer)).exists():
                raise ValueError(f'repair would replace an acceptance test: {source.name}')
        shutil.copytree(answer, folder, dirs_exist_ok=True)
        # Overlays may preserve older mtimes. A fresh target always compiles the repair.
        green, log = suite(folder, cargo, base/'repair-target', timeout)
        if green:
            raise ValueError('repair failed\n' + log[-8000:])
        if 'wide = []' in (folder/'Cargo.toml').read_text(encoding='utf-8'):
            green, log = suite(folder, cargo+['--features', 'wide'], base/'repair-target', timeout)
            if green:
                raise ValueError('wide repair failed\n' + log[-8000:])

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--catalog', type=Path, default=ROOT/'course/catalog.tsv')
    ap.add_argument('--from-position', type=int, default=1)
    ap.add_argument('--through-position', type=int, default=3000)
    ap.add_argument('--repairs', type=Path)
    ap.add_argument('--jobs', type=int, default=4)
    ap.add_argument('--offline', action='store_true')
    opt = ap.parse_args()
    if not 1 <= opt.from_position <= opt.through_position <= 3000 or opt.jobs < 1:
        ap.error('use an ordered position range inside 1..3000 and a positive job count')
    with opt.catalog.open(encoding='utf-8', newline='') as catalog:
        rows = list(csv.DictReader(catalog, delimiter='\t'))
    # The first 50 and the three tool-specific pilots have their own symptom verifiers.
    selected = [r for r in rows if opt.from_position <= int(r['position']) <= opt.through_position
                and int(r['id'][5:]) not in {*range(11,61),1509,2090,2560}]
    errors = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=opt.jobs) as pool:
        futures = {pool.submit(verify,r,opt.repairs,opt.offline):r for r in selected}
        for future in concurrent.futures.as_completed(futures):
            row=futures[future]
            try:
                future.result()
            except Exception as ex:
                errors.append(row['id'])
                print(f"{row['id']} FAILED\n{ex}", flush=True)
    print(f"{len(selected)-len(errors)}/{len(selected)} verified. Failures: {' '.join(errors)}")
    raise SystemExit(bool(errors))

if __name__=='__main__':
    main()
