# Spec: High-Performance Orchestration

## Overview

Scanning 70+ projects sequentially involves significant blocking I/O (filesystem
stats) and subprocess overhead (executing `git status`). This track introduces
multi-core parallelism to make ecosystem-wide operations near-instant.

## Requirements

- **Threaded Scanning:** Parallelize `discovery::scan_all_projects`.
- **Parallel Batch Execution:** Multi-thread the `toad do` command loop.
- **Output Coherence:** Prevent stdout/stderr interleaving from multiple
  threads.
- **Graceful Shutdown:** Support `Ctrl-C` to kill all parallel tasks.

## Design

1. **Crate Selection:** Integrate `rayon` for data-parallelism and `indicatif`
   for high-quality, thread-safe progress reporting.
2. **Parallel Iterators:** Convert existing `for` loops to `.into_par_iter()`.
3. **Result Buffering:** In `toad do`, collect `OpResult` objects from parallel
   tasks and print them in a single sorted block at the end, or use a
   `MultiProgress` bar to show live status per thread.
4. **Resource Limits:** Bound the thread pool to the number of logical cores or
   a configurable `TOAD_THREADS` variable.
