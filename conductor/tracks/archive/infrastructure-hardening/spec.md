# Spec: Infrastructure Hardening

## Overview

Refine the v0.4.0 implementation to address architectural fragile points and
performance bottlenecks identified in principal review.

## Requirements

1. **Parallel Stream Discovery:** Remove serial pre-scan in `scan_all_projects`.
2. **Deterministic Hashing:** Standardize metadata scanning and document
   order-dependency.
3. **UI Throttling:** Reduce overhead of progress bar updates in parallel loops.
4. **Test Determinism:** Remove timing reliance in integration tests.

## Design

- Use `ParallelBridge` in `discovery`.
- Move high-value file list to a crate-level constant in `toad-core`.
- Optimize `indicatif` usage in `bin/toad`.
