#!/usr/bin/env python3
"""
Generate reference bitround outputs for Python using numcodecs.
Reads input data from testdata/inputs/ and writes encoded outputs to testdata/python/
"""

import struct
import os
from pathlib import Path

import numpy as np
from numcodecs import BitRound

TESTDATA_DIR = Path(__file__).parent.parent / "testdata"
INPUTS_DIR = TESTDATA_DIR / "inputs"
OUTPUT_DIR = TESTDATA_DIR / "python"

INPUT_FILES_F32 = [
    "zeros_f32.bin",
    "constants_f32.bin",
    "random_f32.bin",
    "edge_f32.bin",
]

INPUT_FILES_F64 = [
    "zeros_f64.bin",
    "constants_f64.bin",
    "random_f64.bin",
    "edge_f64.bin",
]

NBITS_F32 = [1, 8, 16, 23]
NBITS_F64 = [1, 16, 32, 52]


def load_f32_from_binary(filepath: Path) -> np.ndarray:
    """Load f32 array from binary file (little-endian IEEE 754)."""
    with open(filepath, "rb") as f:
        data = f.read()
    num_floats = len(data) // 4
    return np.frombuffer(data, dtype=np.float32, count=num_floats)


def load_f64_from_binary(filepath: Path) -> np.ndarray:
    """Load f64 array from binary file (little-endian IEEE 754)."""
    with open(filepath, "rb") as f:
        data = f.read()
    num_floats = len(data) // 8
    return np.frombuffer(data, dtype=np.float64, count=num_floats)


def save_u32_to_binary(data: np.ndarray, filepath: Path) -> None:
    """Save u32 array to binary file (little-endian)."""
    with open(filepath, "wb") as f:
        f.write(np.asarray(data, dtype=np.uint32).tobytes())


def save_u64_to_binary(data: np.ndarray, filepath: Path) -> None:
    """Save u64 array to binary file (little-endian)."""
    with open(filepath, "wb") as f:
        f.write(np.asarray(data, dtype=np.uint64).tobytes())


def process_f32(input_file: str, nbits: int) -> None:
    """Process a single f32 input file with given nbits."""
    input_path = INPUTS_DIR / input_file
    output_file = input_file.replace(".bin", f"_nbits{nbits}.bin")
    output_path = OUTPUT_DIR / output_file

    data = load_f32_from_binary(input_path)
    codec = BitRound(keepbits=nbits)
    encoded = codec.encode(data)

    save_u32_to_binary(np.frombuffer(encoded, dtype=np.uint32), output_path)
    print(f"  Created {output_file}")


def process_f64(input_file: str, nbits: int) -> None:
    """Process a single f64 input file with given nbits."""
    input_path = INPUTS_DIR / input_file
    output_file = input_file.replace(".bin", f"_nbits{nbits}.bin")
    output_path = OUTPUT_DIR / output_file

    data = load_f64_from_binary(input_path)
    codec = BitRound(keepbits=nbits)
    encoded = codec.encode(data)

    save_u64_to_binary(np.frombuffer(encoded, dtype=np.uint64), output_path)
    print(f"  Created {output_file}")


def main():
    print("=== Generating Python Reference Data ===")
    print()

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    print("Processing f32 inputs...")
    for input_file in INPUT_FILES_F32:
        for nbits in NBITS_F32:
            process_f32(input_file, nbits)

    print()
    print("Processing f64 inputs...")
    for input_file in INPUT_FILES_F64:
        for nbits in NBITS_F64:
            process_f64(input_file, nbits)

    print()
    print(f"Reference outputs saved to {OUTPUT_DIR}/")


if __name__ == "__main__":
    main()
