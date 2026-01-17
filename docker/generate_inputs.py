#!/usr/bin/env python3
"""Generate test input arrays for verification testing.

This script creates binary test files with various f32 and f64 arrays
that can be used by Python, Julia, and Rust verification tests.
"""

import os
import struct
import numpy as np

INPUT_DIR = "/data/inputs"

NBITS_F32 = [1, 8, 16, 24]
NBITS_F64 = [1, 16, 32, 53]

ARRAY_SIZES = {
    "zeros": 1000,
    "constants": 1000,
    "random": 1000,
    "edge": 100,
}


def save_binary(data: np.ndarray, filepath: str):
    """Save numpy array as raw binary file."""
    data.tofile(filepath)


def generate_f32_test_arrays():
    """Generate f32 test input arrays."""
    arrays = {}

    arrays["zeros_f32"] = np.zeros(ARRAY_SIZES["zeros"], dtype=np.float32)

    arrays["constants_f32"] = np.full(ARRAY_SIZES["constants"], 1.5, dtype=np.float32)

    np.random.seed(42)
    arrays["random_f32"] = np.random.randn(ARRAY_SIZES["random"]).astype(np.float32)

    arrays["edge_f32"] = np.array(
        [
            0.0,
            -0.0,
            1.0,
            -1.0,
            np.finfo(np.float32).max,
            np.finfo(np.float32).min,
            np.finfo(np.float32).eps,
            np.nan,
            np.inf,
            -np.inf,
        ]
        + [np.finfo(np.float32).smallest_subnormal] * 10
        + list(np.random.randn(80).astype(np.float32)),
        dtype=np.float32,
    )

    return arrays


def generate_f64_test_arrays():
    """Generate f64 test input arrays."""
    arrays = {}

    arrays["zeros_f64"] = np.zeros(ARRAY_SIZES["zeros"], dtype=np.float64)

    arrays["constants_f64"] = np.full(ARRAY_SIZES["constants"], 1.5, dtype=np.float64)

    np.random.seed(42)
    arrays["random_f64"] = np.random.randn(ARRAY_SIZES["random"]).astype(np.float64)

    arrays["edge_f64"] = np.array(
        [
            0.0,
            -0.0,
            1.0,
            -1.0,
            np.finfo(np.float64).max,
            np.finfo(np.float64).min,
            np.finfo(np.float64).eps,
            np.nan,
            np.inf,
            -np.inf,
        ]
        + [np.finfo(np.float64).smallest_subnormal] * 10
        + list(np.random.randn(80).astype(np.float64)),
        dtype=np.float64,
    )

    return arrays


def generate_inputs():
    """Generate all test input arrays."""
    os.makedirs(INPUT_DIR, exist_ok=True)

    f32_arrays = generate_f32_test_arrays()
    f64_arrays = generate_f64_test_arrays()

    print("Generating test input arrays")
    print("=" * 60)

    for name, data in f32_arrays.items():
        output_path = os.path.join(INPUT_DIR, f"{name}.bin")
        save_binary(data, output_path)
        print(f"Saved: {name}.bin ({len(data)} elements, {data.nbytes} bytes)")

    for name, data in f64_arrays.items():
        output_path = os.path.join(INPUT_DIR, f"{name}.bin")
        save_binary(data, output_path)
        print(f"Saved: {name}.bin ({len(data)} elements, {data.nbytes} bytes)")

    print("=" * 60)
    print(f"Test inputs saved to {INPUT_DIR}")


if __name__ == "__main__":
    generate_inputs()
