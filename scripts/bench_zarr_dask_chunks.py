#!/usr/bin/env python3
"""
Benchmark xarray.to_zarr write + xarray.open_zarr read for a 3D array
shaped like one month of hourly global 0.25-degree data:

    shape       = (24*31, 1440, 721)   # (time, lon, lat)
    zarr chunks = (24*31, 12, 12)      # full time, small spatial tiles

The write side is parametrised by a multiplier `x` for the spatial dask
chunks: dask chunks = (24*31, 12*x, 12*x) for x in {1, 2, 4, 8}. With x=1
each dask chunk maps 1:1 to a zarr chunk; larger x packs more zarr chunks
per dask task, reducing task count at the cost of per-task memory.

The read side always uses dask chunks = (1, 1440, 721) — a single
timestep, full spatial map — so it crosses many zarr chunks per dask
read-task. Read is timed with `.mean().compute()` to force I/O.

Inspired by the icechunk GLAD ingest guide
(https://icechunk.io/en/stable/guides/ingestion/glad-ingest/), which
recommends aligning dask write chunks to integer multiples of zarr
chunks for parallel ingest.
"""

from __future__ import annotations

import argparse
import json
import shutil
import time
from pathlib import Path

import dask
import dask.array as da
import numpy as np
import xarray as xr
import zarr


SHAPE = (24 * 31, 1440, 721)
ZARR_CHUNKS = (24 * 31, 12, 12)
READ_CHUNKS = (1, 1440, 721)
DASK_X_VALUES = (1, 2, 4, 8)


def build_dataset(x: int) -> xr.Dataset:
    """Synthetic float32 ds with dask chunks = (24*31, 12*x, 12*x)."""
    dchunks = (24 * 31, 12 * x, 12 * x)
    arr = da.random.default_rng(42).standard_normal(
        size=SHAPE, chunks=dchunks, dtype=np.float32
    )
    return xr.Dataset(
        {"t2m": (("time", "lon", "lat"), arr)},
        coords={
            "time": np.arange(SHAPE[0]),
            "lon": np.linspace(-180, 180, SHAPE[1], endpoint=False),
            "lat": np.linspace(-90, 90, SHAPE[2]),
        },
    )


def count_tasks(ds: xr.Dataset) -> int:
    """Number of tasks in the dask graph backing ds.t2m."""
    return len(dict(ds["t2m"].data.__dask_graph__()))


def time_write(ds: xr.Dataset, store_path: Path) -> float:
    """Write ds to a fresh zarr store with fixed zarr chunks; return wall seconds."""
    if store_path.exists():
        shutil.rmtree(store_path)
    encoding = {"t2m": {"chunks": ZARR_CHUNKS}}
    t0 = time.perf_counter()
    ds.to_zarr(store_path, mode="w", encoding=encoding, consolidated=True)
    return time.perf_counter() - t0


def time_read(store_path: Path, n_timesteps: int) -> tuple[float, int, int]:
    """Open store with read dask chunks of (1, 1440, 721) and compute the mean
    over the first ``n_timesteps`` timesteps. Returns (seconds, total_task_count,
    sliced_task_count). The full graph task count counts all dask tasks the
    user *would* see when doing a full-dataset read; the sliced count is what
    actually runs."""
    ds = xr.open_zarr(
        store_path,
        chunks={"time": READ_CHUNKS[0], "lon": READ_CHUNKS[1], "lat": READ_CHUNKS[2]},
        consolidated=True,
    )
    n_tasks_full = len(dict(ds["t2m"].data.__dask_graph__()))
    sliced = ds["t2m"].isel(time=slice(0, n_timesteps))
    n_tasks_sliced = len(dict(sliced.data.__dask_graph__()))
    t0 = time.perf_counter()
    sliced.mean().compute()
    return time.perf_counter() - t0, n_tasks_full, n_tasks_sliced


def store_size_bytes(store_path: Path) -> int:
    return sum(p.stat().st_size for p in store_path.rglob("*") if p.is_file())


def run(out_dir: Path, repeats: int, read_timesteps: int) -> dict:
    out_dir.mkdir(parents=True, exist_ok=True)
    results = {
        "shape": SHAPE,
        "zarr_chunks": ZARR_CHUNKS,
        "read_chunks": READ_CHUNKS,
        "dask_x_values": list(DASK_X_VALUES),
        "repeats": repeats,
        "dask_scheduler": dask.config.get("scheduler", "threads"),
        "runs": [],
    }

    for x in DASK_X_VALUES:
        dchunks = (24 * 31, 12 * x, 12 * x)
        ds = build_dataset(x)
        n_write_tasks = count_tasks(ds)

        # zarr chunks per dask chunk along each spatial dim:
        # dask=12*x divided by zarr=12 -> x along each, so x*x zarr chunks per dask chunk
        zarr_per_dask = x * x
        n_zarr_chunks = (1) * (1440 // 12) * (721 // 12 + (1 if 721 % 12 else 0))

        store = out_dir / f"store_x{x}.zarr"
        write_times = []
        for r in range(repeats):
            print(f"  x={x} write repeat {r + 1}/{repeats} ...", flush=True)
            ds_r = build_dataset(x)  # rebuild to avoid cached compute
            wt = time_write(ds_r, store)
            write_times.append(wt)
            print(f"    write {wt:.2f}s", flush=True)
        size_mb = store_size_bytes(store) / (1024 * 1024)

        read_times = []
        n_read_tasks_full = None
        n_read_tasks_sliced = None
        for r in range(repeats):
            print(f"  x={x} read repeat {r + 1}/{repeats} ...", flush=True)
            t, n_read_tasks_full, n_read_tasks_sliced = time_read(store, read_timesteps)
            read_times.append(t)
            print(
                f"    read {t:.2f}s "
                f"(full graph {n_read_tasks_full} tasks, "
                f"sliced graph {n_read_tasks_sliced} tasks)",
                flush=True,
            )

        run_result = {
            "x": x,
            "dask_chunks": list(dchunks),
            "n_write_tasks": n_write_tasks,
            "n_zarr_chunks_per_dask_chunk": zarr_per_dask,
            "approx_n_zarr_chunks_total": n_zarr_chunks,
            "n_read_tasks_full_graph": n_read_tasks_full,
            "n_read_tasks_sliced_graph": n_read_tasks_sliced,
            "read_timesteps": read_timesteps,
            "write_seconds": {
                "median": float(np.median(write_times)),
                "min": float(np.min(write_times)),
                "max": float(np.max(write_times)),
                "all": [float(t) for t in write_times],
            },
            "read_seconds": {
                "median": float(np.median(read_times)),
                "min": float(np.min(read_times)),
                "max": float(np.max(read_times)),
                "all": [float(t) for t in read_times],
            },
            "store_size_mb": size_mb,
        }
        results["runs"].append(run_result)
        print(
            f"x={x:>2}  dask={dchunks}  write_tasks={n_write_tasks:>5}  "
            f"write={run_result['write_seconds']['median']:.2f}s  "
            f"read_tasks_full={n_read_tasks_full:>5}  "
            f"read[{read_timesteps}t]={run_result['read_seconds']['median']:.2f}s  "
            f"size={size_mb:.0f} MB",
            flush=True,
        )

    return results


def format_markdown(results: dict) -> str:
    lines = []
    lines.append("# zarr/dask chunk benchmark")
    lines.append("")
    lines.append(f"- array shape: `{tuple(results['shape'])}` float32")
    lines.append(f"- zarr chunks (write): `{tuple(results['zarr_chunks'])}`")
    lines.append(f"- read dask chunks: `{tuple(results['read_chunks'])}`")
    lines.append(f"- repeats: {results['repeats']}")
    lines.append(f"- dask scheduler: `{results['dask_scheduler']}`")
    if results["runs"]:
        lines.append(f"- read timesteps materialised: {results['runs'][0]['read_timesteps']}")
    lines.append("")
    lines.append(
        "| x | dask chunks | write tasks | zarr/dask | write (s, median) | "
        "read full graph tasks | read sliced graph tasks | read (s, median) | store (MB) |"
    )
    lines.append(
        "|---|---|---|---|---|---|---|---|---|"
    )
    for r in results["runs"]:
        lines.append(
            f"| {r['x']} | {tuple(r['dask_chunks'])} | {r['n_write_tasks']} | "
            f"{r['n_zarr_chunks_per_dask_chunk']} | "
            f"{r['write_seconds']['median']:.2f} | "
            f"{r['n_read_tasks_full_graph']} | "
            f"{r['n_read_tasks_sliced_graph']} | "
            f"{r['read_seconds']['median']:.2f} | "
            f"{r['store_size_mb']:.0f} |"
        )
    return "\n".join(lines) + "\n"


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--out-dir", type=Path, default=Path("/tmp/zarr_bench"))
    p.add_argument("--repeats", type=int, default=2)
    p.add_argument(
        "--read-timesteps",
        type=int,
        default=10,
        help=(
            "How many leading timesteps to materialise during the read benchmark. "
            "Each read produces a (1, 1440, 721) dask chunk; a full read of all "
            f"{SHAPE[0]} timesteps is dominated by re-reading every spatial zarr "
            "chunk per timestep, so the default keeps the benchmark tractable."
        ),
    )
    p.add_argument("--json", action="store_true", help="Print JSON instead of markdown")
    p.add_argument(
        "--results-json",
        type=Path,
        default=None,
        help="Optional path to dump results JSON",
    )
    p.add_argument(
        "--results-md",
        type=Path,
        default=None,
        help="Optional path to dump results markdown",
    )
    args = p.parse_args()

    results = run(args.out_dir, args.repeats, args.read_timesteps)

    md = format_markdown(results)
    if args.json:
        print(json.dumps(results, indent=2))
    else:
        print()
        print(md)

    if args.results_json:
        args.results_json.write_text(json.dumps(results, indent=2))
    if args.results_md:
        args.results_md.write_text(md)


if __name__ == "__main__":
    main()
