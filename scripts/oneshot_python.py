#!/usr/bin/env python3
"""One-shot bitround for a single 3D array. Prints `encode_seconds=<float>`."""
import sys
import time

import numpy as np
from numcodecs import BitRound


def main() -> int:
    edge = int(sys.argv[1])
    keepbits = int(sys.argv[2])

    rng = np.random.default_rng(42)
    data = (
        273.0 + rng.random((edge, edge, edge), dtype=np.float64) * 20.0
    ).astype(np.float32).flatten()

    codec = BitRound(keepbits=keepbits)
    codec.encode(data)  # warmup
    t0 = time.perf_counter()
    out = codec.encode(data)
    t1 = time.perf_counter()
    sys.stdout.write(f"encode_seconds={t1 - t0}\nout_bytes={out.nbytes}\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
