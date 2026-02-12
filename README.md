# Aether eBPF I/O Monitor

A lightweight eBPF-based I/O latency monitoring tool built with the Aya eBPF framework for the AetherMeters project.

## Overview

This project implements an eBPF program that tracks I/O operations and measures their latency using kprobes and kretprobes. It provides real-time insights into I/O performance by capturing the start and end times of I/O operations.

## Features

- **Low overhead**: Uses eBPF for efficient kernel-space monitoring
- **Real-time I/O latency tracking**: Measures the time taken for I/O operations
- **Process-level granularity**: Tracks latency per process ID
- **High performance**: Minimal impact on system performance
- **Rust-based implementation**: Leverages the Aya eBPF framework for safe and idiomatic eBPF development

## Technical Stack

- **Rust**: Main programming language
- **Aya eBPF**: Framework for eBPF development in Rust
- **eBPF**: Kernel-level technology for efficient system monitoring
- **kprobes/kretprobes**: Linux kernel hooks for capturing function entry/exit

## Project Structure

```
aether-ebpf/
├── .cargo/             # Cargo configuration
│   └── config.toml     # Build target configuration for eBPF
├── src/                # Source code
│   └── main.rs         # Main eBPF program
├── Cargo.toml          # Project dependencies
├── Cargo.lock          # Locked dependencies
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

## Getting Started

### Prerequisites

- **Rust**: Version 1.60 or higher
- **Linux kernel**: Version 4.18 or higher (with eBPF support)
- **Aya eBPF dependencies**: Follow the [Aya documentation](https://aya-rs.dev/book/getting-started/setup.html) for setup

### Building

1. Clone the repository:

   ```bash
   git clone https://github.com/jinzhao-rjb/aether-io-monitor.git
   cd aether-io-monitor
   ```

2. Build the eBPF program:

   ```bash
   cargo build
   ```

### Usage

This eBPF program is designed to be loaded and controlled by a userspace application. It provides two main probes:

- **aether_io_trace**: Captures the start time of I/O operations
- **aether_io_ret**: Captures the end time and calculates latency

The measured latencies are stored in an eBPF map (`IO_LATENCY`) indexed by process ID.

### Example Userspace Integration

To use this eBPF program, you'll need to create a userspace application that:

1. Loads the eBPF program
2. Attaches the kprobes to the appropriate kernel functions
3. Reads latency data from the eBPF maps
4. Processes and displays the collected data

## Kernel Functions to Probe

The current implementation provides the probe functions, but you'll need to attach them to specific kernel functions based on your needs. Common I/O functions to probe include:

- `vfs_read`/`vfs_write` for file I/O
- `blk_start_request`/`blk_end_request` for block device I/O
- `io_submit`/`io_complete` for asynchronous I/O

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Aya eBPF Framework](https://github.com/aya-rs/aya) for making eBPF development in Rust possible
- The Linux kernel team for eBPF technology
- The Rust community for their excellent tools and libraries

## Contact

For questions or feedback, please open an issue on GitHub.
