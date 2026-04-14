#!/usr/bin/env python3
"""
Cross-implementation time + peak-memory benchmark.

For each (implementation, size) it spawns a fresh subprocess that performs
exactly one warmup encode and one measured encode, then exits. Wall time and
peak resident set size of the whole process are captured via `/usr/bin/time -l`
(macOS) or `/usr/bin/time -v` (GNU/Linux). The encode-only time is parsed from
the child's stdout (`encode_seconds=...`).

Note: peak RSS is whole-process, so Python/Julia runtime startup is included.
That overhead is real cost the user pays per invocation; we report it as-is
and document it next to the table.

Usage:
  python scripts/bench_memory.py [--keepbits 16] [--sizes 10,100] [--repeats 3]
"""

from __future__ import annotations

import argparse
import platform
import re
import shutil
import statistics
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
RUST_BIN = REPO_ROOT / "target" / "release" / "bench-oneshot"
PY_SCRIPT = REPO_ROOT / "scripts" / "oneshot_python.py"
JL_SCRIPT = REPO_ROOT / "scripts" / "oneshot_julia.jl"
PY_INTERP = REPO_ROOT / "venv" / "bin" / "python"


@dataclass
class Sample:
    encode_seconds: float
    wall_seconds: float
    peak_rss_bytes: int


def build_rust() -> None:
    if RUST_BIN.exists():
        return
    print("Building Rust bench-oneshot binary...", file=sys.stderr)
    subprocess.run(
        ["cargo", "build", "--release", "--bin", "bench-oneshot"],
        cwd=REPO_ROOT,
        check=True,
    )


def parse_time_output(stderr: str) -> tuple[float, int]:
    """Parse `/usr/bin/time -l` (BSD/macOS) or `-v` (GNU) output for wall + RSS."""
    wall = None
    rss = None

    # macOS BSD time -l format
    m = re.search(r"^\s*([\d.]+)\s+real\b", stderr, re.MULTILINE)
    if m:
        wall = float(m.group(1))
    m = re.search(r"^\s*(\d+)\s+maximum resident set size", stderr, re.MULTILINE)
    if m:
        rss = int(m.group(1))  # bytes on macOS

    # GNU time -v format
    if wall is None:
        m = re.search(r"Elapsed \(wall clock\) time.*?:\s*([\d:.]+)", stderr)
        if m:
            t = m.group(1)
            parts = [float(p) for p in t.split(":")]
            if len(parts) == 3:
                wall = parts[0] * 3600 + parts[1] * 60 + parts[2]
            elif len(parts) == 2:
                wall = parts[0] * 60 + parts[1]
            else:
                wall = parts[0]
    if rss is None:
        m = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", stderr)
        if m:
            rss = int(m.group(1)) * 1024  # KB → bytes

    if wall is None or rss is None:
        raise RuntimeError(f"could not parse time output:\n{stderr}")
    return wall, rss


def run_one(cmd: list[str]) -> Sample:
    time_bin = "/usr/bin/time"
    flag = "-l" if platform.system() == "Darwin" else "-v"
    proc = subprocess.run(
        [time_bin, flag, *cmd],
        capture_output=True,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"command failed: {' '.join(cmd)}\nstdout:\n{proc.stdout}\nstderr:\n{proc.stderr}"
        )
    m = re.search(r"encode_seconds=([\d.eE+-]+)", proc.stdout)
    if not m:
        raise RuntimeError(f"missing encode_seconds in stdout:\n{proc.stdout}")
    encode_s = float(m.group(1))
    wall_s, rss_b = parse_time_output(proc.stderr)
    return Sample(encode_seconds=encode_s, wall_seconds=wall_s, peak_rss_bytes=rss_b)


def fmt_bytes(b: int) -> str:
    for unit, scale in (("GB", 1 << 30), ("MB", 1 << 20), ("KB", 1 << 10)):
        if b >= scale:
            return f"{b / scale:.1f} {unit}"
    return f"{b} B"


def fmt_seconds(s: float) -> str:
    if s < 1e-3:
        return f"{s * 1e6:.1f} μs"
    if s < 1.0:
        return f"{s * 1e3:.2f} ms"
    return f"{s:.3f} s"


def aggregate(samples: list[Sample]) -> tuple[float, float, int]:
    enc = statistics.median(s.encode_seconds for s in samples)
    wall = statistics.median(s.wall_seconds for s in samples)
    rss = max(s.peak_rss_bytes for s in samples)
    return enc, wall, rss


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--keepbits", type=int, default=16)
    parser.add_argument("--sizes", type=str, default="10,100")
    parser.add_argument("--repeats", type=int, default=3)
    parser.add_argument("--no-julia", action="store_true")
    args = parser.parse_args()

    build_rust()
    if not PY_INTERP.exists():
        print(f"venv python not found at {PY_INTERP}", file=sys.stderr)
        return 1

    sizes = [int(s) for s in args.sizes.split(",") if s.strip()]
    impls: list[tuple[str, list[str]]] = [
        ("python (numcodecs)", [str(PY_INTERP), str(PY_SCRIPT)]),
        ("rust (this repo)", [str(RUST_BIN)]),
    ]
    if not args.no_julia and shutil.which("julia"):
        impls.append(("julia (BitInformation.jl)", ["julia", str(JL_SCRIPT)]))
    elif not args.no_julia:
        print("julia not on PATH; skipping julia row", file=sys.stderr)

    print(
        f"# bitround time + peak RSS — keepbits={args.keepbits}, "
        f"repeats={args.repeats}, platform={platform.platform()}",
        file=sys.stderr,
    )

    rows: list[tuple[str, int, float, float, int]] = []
    for name, base in impls:
        for edge in sizes:
            print(f"\n→ {name}  size={edge}^3", file=sys.stderr)
            samples: list[Sample] = []
            for _ in range(args.repeats):
                s = run_one([*base, str(edge), str(args.keepbits)])
                samples.append(s)
                print(
                    f"   encode={fmt_seconds(s.encode_seconds)}  "
                    f"wall={fmt_seconds(s.wall_seconds)}  "
                    f"rss={fmt_bytes(s.peak_rss_bytes)}",
                    file=sys.stderr,
                )
            enc, wall, rss = aggregate(samples)
            rows.append((name, edge, enc, wall, rss))

    print()
    print("## Time + Peak Memory")
    print()
    print(
        f"keepbits={args.keepbits}, repeats={args.repeats}, "
        f"medians for time, max for RSS, peak RSS includes runtime startup."
    )
    print()
    print(
        "| Implementation | Size | Encode time (median) | Wall time (median) | Peak RSS |"
    )
    print(
        "|----------------|------|----------------------|--------------------|----------|"
    )
    for name, edge, enc, wall, rss in rows:
        print(
            f"| {name} | {edge}^3 | {fmt_seconds(enc)} | {fmt_seconds(wall)} | {fmt_bytes(rss)} |"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
