#!/usr/bin/env python3
"""
Cross-implementation equivalence check for bitround.

Generates reference f32 arrays, encodes them with:
  - numcodecs.BitRound (Python)
  - ./target/release/encode-file  (Rust — this repo)
  - scripts/encode_file.jl         (Julia — BitInformation.jl)

All three produce a stream of u32 values. This script asserts they are
bitwise identical on the same input. Any mismatch fails with a diff.

Usage:
  python scripts/verify_equivalence.py [--keepbits 16] [--sizes 10,100,1000]
"""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

import numpy as np
from numcodecs import BitRound


REPO_ROOT = Path(__file__).resolve().parents[1]
RUST_BIN = REPO_ROOT / "target" / "release" / "encode-file"
JULIA_SCRIPT = REPO_ROOT / "scripts" / "encode_file.jl"


def build_rust_binary() -> None:
    if RUST_BIN.exists():
        return
    print("Building Rust encode-file binary...", file=sys.stderr)
    subprocess.run(
        ["cargo", "build", "--release", "--bin", "encode-file"],
        cwd=REPO_ROOT,
        check=True,
    )


def generate_reference(edge: int, seed: int = 42) -> np.ndarray:
    rng = np.random.default_rng(seed)
    return (273.0 + rng.random((edge, edge, edge), dtype=np.float64) * 20.0).astype(np.float32)


def encode_numcodecs(data: np.ndarray, keepbits: int) -> np.ndarray:
    codec = BitRound(keepbits=keepbits)
    out = codec.encode(data.flatten())
    return np.frombuffer(out.tobytes(), dtype=np.uint32)


def encode_rust(input_path: Path, output_path: Path, keepbits: int) -> np.ndarray:
    subprocess.run(
        [
            str(RUST_BIN),
            "--input",
            str(input_path),
            "--output",
            str(output_path),
            "--keepbits",
            str(keepbits),
        ],
        check=True,
    )
    return np.fromfile(output_path, dtype="<u4")


def encode_julia(input_path: Path, output_path: Path, keepbits: int) -> np.ndarray:
    if shutil.which("julia") is None:
        raise RuntimeError("julia not found on PATH; install julia to run equivalence check")
    julia_cmd = ["julia"]
    if (REPO_ROOT / "Project.toml").exists():
        julia_cmd.append("--project=.")
    subprocess.run(
        julia_cmd
        + [
            str(JULIA_SCRIPT),
            "--input",
            str(input_path),
            "--output",
            str(output_path),
            "--keepbits",
            str(keepbits),
        ],
        cwd=REPO_ROOT,
        check=True,
    )
    return np.fromfile(output_path, dtype="<u4")


def diff_report(name_a: str, a: np.ndarray, name_b: str, b: np.ndarray) -> str:
    assert a.shape == b.shape
    mismatches = np.flatnonzero(a != b)
    if mismatches.size == 0:
        return ""
    first = mismatches[:5]
    lines = [
        f"  {name_a} vs {name_b}: {mismatches.size} / {a.size} mismatches",
    ]
    for i in first:
        lines.append(f"    [{i}] {name_a}=0x{a[i]:08x}  {name_b}=0x{b[i]:08x}")
    return "\n".join(lines)


def verify_size(edge: int, keepbits: int, include_julia: bool) -> bool:
    print(f"\n=== size {edge}^3  keepbits={keepbits} ===", file=sys.stderr)
    data = generate_reference(edge)
    flat = data.flatten()

    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        input_path = tmp_path / "input.bin"
        flat.astype("<f4").tofile(input_path)

        py = encode_numcodecs(flat, keepbits)
        rust = encode_rust(input_path, tmp_path / "rust.bin", keepbits)

        print(f"  python (numcodecs):  {py.size} elements", file=sys.stderr)
        print(f"  rust (this repo):    {rust.size} elements", file=sys.stderr)

        ok = True
        d = diff_report("python", py, "rust", rust)
        if d:
            print(d, file=sys.stderr)
            ok = False

        if include_julia:
            jl = encode_julia(input_path, tmp_path / "julia.bin", keepbits)
            print(f"  julia (BitInformation.jl): {jl.size} elements", file=sys.stderr)
            d = diff_report("python", py, "julia", jl)
            if d:
                print(d, file=sys.stderr)
                ok = False
            d = diff_report("rust", rust, "julia", jl)
            if d:
                print(d, file=sys.stderr)
                ok = False

    if ok:
        print(f"  OK — all implementations bitwise identical", file=sys.stderr)
    return ok


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--keepbits", type=int, default=16)
    parser.add_argument("--sizes", type=str, default="10,100")
    parser.add_argument(
        "--no-julia",
        action="store_true",
        help="skip julia (if BitInformation.jl or julia is not installed)",
    )
    args = parser.parse_args()

    build_rust_binary()

    sizes = [int(s) for s in args.sizes.split(",") if s.strip()]
    all_ok = True
    for edge in sizes:
        all_ok &= verify_size(edge, args.keepbits, include_julia=not args.no_julia)

    if not all_ok:
        print("\nFAIL: implementations disagree", file=sys.stderr)
        return 1
    print("\nPASS: all implementations bitwise identical on all sizes", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
