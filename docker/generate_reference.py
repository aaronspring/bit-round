#!/usr/bin/env python3
"""Generate reference bitround outputs for verification testing.

This script runs the Python numcodecs bitround implementation and saves
the encoded outputs as binary files for comparison with Rust implementation.
"""

import os
import struct
import numpy as np
from numcodecs import Bitround

INPUT_DIR = "/data/inputs"
OUTPUT_DIR = "/data/outputs"

NBITS_F32 = [1, 8, 16, 24]
NBITS_F64 = [1, 16, 32, 53]


def save_binary(data: np.ndarray, filepath: str):
    """Save numpy array as raw binary file."""
    data.tofile(filepath)


def load_input(filepath: str) -> np.ndarray:
    """Load input array from binary file."""
    name = os.path.basename(filepath)

    if name.endswith("_f32.bin"):
        return np.fromfile(filepath, dtype=np.float32)
    elif name.endswith("_f64.bin"):
        return np.fromfile(filepath, dtype=np.float64)
    else:
        raise ValueError(f"Unknown file type: {name}")


def run_python_verification():
    """Run Python numcodecs bitround and save reference outputs."""
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    input_files = [f for f in os.listdir(INPUT_DIR) if f.endswith(".bin")]

    print("Python numcodecs bitround reference output generation")
    print("=" * 60)

    for filename in sorted(input_files):
        input_path = os.path.join(INPUT_DIR, filename)
        data = load_input(input_path)

        if data.dtype == np.float32:
            nbits_values = NBITS_F32
        else:
            nbits_values = NBITS_F64

        for nbits in nbits_values:
            bitround = Bitround(nbits=nbits)
            encoded = bitround.encode(data)

            base_name = filename.replace(".bin", "")
            output_path = os.path.join(OUTPUT_DIR, f"{base_name}_nbits{nbits}.bin")
            save_binary(encoded, output_path)
            print(f"Saved: {output_path}")

    print("=" * 60)
    print(f"Reference outputs saved to {OUTPUT_DIR}")


if __name__ == "__main__":
    run_python_verification()
