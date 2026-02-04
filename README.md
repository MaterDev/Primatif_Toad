# Code System

This repository serves as the **Control Plane** for my local development environment. 

It uses a "Zero-Conflict Overlay" strategy:
- **Root:** A Rust application for system management and automation.
- **`projects/`:** A folder containing all actual software projects (70+ repositories). These are **ignored** by this root Git repo.
- **`scripts/`:** Shell scripts for quick tasks.
- **`work_orders/`:** Logs of major infrastructure changes.

## Setup

This root directory is a Rust workspace. 

```bash
# Run the main tool (currently Hello World)
cargo run
```

## Structure

- `src/`: Rust source code for the control plane CLI/tools.
- `projects/`: The "User Space" where all project repositories live.
- `scripts/`: Legacy or simple shell automation.
- `doc/`: Documentation for this system.
