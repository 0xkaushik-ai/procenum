# procenum

A minimal **process enumerator** for Linux, written in Rust as a systems-programming learning exercise.

It lists every running process by reading the kernel's `/proc` virtual filesystem — the same source `ps`, `top`, and Task Manager use. It only **reads** process metadata; it changes nothing on the system.

## What it does

For every running process, `procenum`:

1. Lists the numeric folders under `/proc` (each one is a live process; the folder name is its **PID**).
2. Reads `/proc/<pid>/comm` to get the process **name**.
3. Prints a `PID -> Name` list and a total count.
4. Reports basic timing (how long the scan took, and how much of that was spent reading files).

Example output:

```
PID: 1, Name: systemd
PID: 487, Name: firefox
PID: 1452, Name: bash
...
Total processes found: 486
Time inside read_name: 2.1ms
Total time taken: 3.5ms
```

## How it works

Linux exposes every process as a folder under `/proc`. These files are **virtual** — the kernel generates them live, on each read; nothing is stored on disk.

| Path | Provides |
|------|----------|
| `/proc/<pid>/` | One folder per running process |
| `/proc/<pid>/comm` | Short process name (e.g. `firefox`) |

The program filters `/proc` entries with `name.parse::<u32>()` to keep only real PIDs (numeric names), skipping info files like `/proc/cpuinfo`.

## Build & run

Requires the Rust toolchain ([rustup](https://rustup.rs)).

```bash
# Debug build (fast to compile, slow to run)
cargo run

# Release build (optimized — use this for timing)
cargo run --release
```

## Notes on performance

This tool is **I/O-bound**: nearly all its time is spent on syscalls reading `/proc` files, not on CPU work. Because a syscall costs the same regardless of language, a Rust build is only modestly faster than an equivalent Python script here (~3x on the author's machine), rather than the large gap seen in CPU-heavy workloads. Always benchmark with `--release`.

## Security context

Process enumeration is a foundational technique on both sides of security:

- **Defensive:** EDR/antivirus tools enumerate processes to spot anomalies (e.g. a document viewer spawning a shell).
- **Offensive:** malware enumerates processes to detect analysis tools or antivirus before acting.

This project studies the mechanism in a benign, read-only form for educational purposes.

## Roadmap

- [ ] Add parent PID (PPID) and process state from `/proc/<pid>/stat`
- [ ] Add process owner (UID) from `/proc/<pid>/status`
- [ ] Filter/search by process name
- [ ] Build a parent -> child process tree
- [ ] Flag suspicious parent/child relationships (a mini detection rule)

## License

Educational project — no license. Use freely for learning.
