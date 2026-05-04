# zarr/dask chunk benchmark

Inspired by the [icechunk GLAD ingest
guide](https://icechunk.io/en/stable/guides/ingestion/glad-ingest/), which
recommends aligning dask write chunks to integer multiples of the on-disk
zarr chunks for parallel ingest. This run measures how that multiplier
affects `xarray.Dataset.to_zarr` wall time and dask graph size, and what
the matching read pattern costs when the read uses a chunking that does
**not** align with the on-disk layout.

- array shape: `(744, 1440, 721)` float32 (~2.9 GB)
- zarr chunks (write, on disk): `(744, 12, 12)`
- read dask chunks: `(1, 1440, 721)` — single timestep, full spatial map
- repeats per cell: 2 (median reported)
- dask scheduler: `threads` (4 cores, local SSD)
- read timesteps materialised: 5 (a full read of all 744 timesteps would
  re-read every spatial zarr chunk per timestep)

## Configuration legend

- `x` — multiplier on the spatial dask chunk: dask chunks `(744, 12*x, 12*x)`
- `zarr/dask` — number of zarr chunks packed into one dask chunk (`x*x`)
- `write tasks` — `len(dataset.t2m.data.__dask_graph__())` before `to_zarr`
- `read full graph tasks` — task count when opening the store with read
  chunks `(1, 1440, 721)` for the *full* `(744, 1440, 721)` array
- `read sliced graph tasks` — task count for the actually-computed
  `isel(time=slice(0, 5)).mean()` graph

## Results

| x | dask chunks | write tasks | zarr/dask | write (s, median) | read full graph tasks | read sliced graph tasks | read (s, median) | store (MB) |
|---|---|---|---|---|---|---|---|---|
| 1 | (744, 12, 12) | 7320 | 1 | 9.12 | 745 | 750 | 8.52 | 2768 |
| 2 | (744, 24, 24) | 1860 | 4 | 6.17 | 745 | 750 | 9.04 | 2768 |
| 4 | (744, 48, 48) | 480 | 16 | 5.01 | 745 | 750 | 9.26 | 2768 |
| 8 | (744, 96, 96) | 120 | 64 | 5.25 | 745 | 750 | 9.23 | 2768 |

## Reading the table

- **Write task count drops 61×** going from `x=1` to `x=8` (7320 → 120) —
  one dask task per `(744, 12*x, 12*x)` chunk. Smaller graphs schedule
  faster.
- **Write wall time** drops from 9.1 s (x=1) to 5.0 s (x=4) and then plateaus
  — graph-overhead dominated at x=1, throughput-bound from x=4 onward
  on this 4-core machine.
- **Store size is identical** (~2.77 GB) across all configurations because
  the on-disk zarr chunking `(744, 12, 12)` is fixed; only the dask write
  task fan-out changes.
- **Read time** is essentially flat (~8.5–9.3 s for 5 timesteps) and
  independent of `x`. The read graph is identical across runs (745 tasks
  for the full array, 750 for the 5-timestep slice + mean) because the
  on-disk chunking is the same in every store. Each read dask chunk has
  to load *every* spatial zarr chunk along its row (zarr chunks span the
  full time dim; the read slices a single time index out of each), so
  this is the worst-case read pattern for this layout.

## Reproducing

```bash
python scripts/bench_zarr_dask_chunks.py --repeats 2 --read-timesteps 5 \
  --out-dir /tmp/zarr_bench \
  --results-md scripts/zarr_dask_chunks_results.md \
  --results-json scripts/zarr_dask_chunks_results.json
```

The full-data read (no `--read-timesteps` cap) would multiply read time
by ~744/5 ≈ 150× under this access pattern; if you want fast time-series
reads, store the data with a `(1, 1440, 721)` zarr chunking instead.
