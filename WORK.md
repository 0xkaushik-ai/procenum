# WORK LOG — procenum

A running log of what this project is, how it's built, and what I've learned.
Project: a benign **process enumerator + detection tool** in Rust, for security learning on Kali Linux.

---

## Status: ✅ Milestone 4 complete — detection rule works

Current capability: lists every running process (PID, name, PPID, state) by reading
`/proc`, and flags suspicious parent -> child pairs (e.g. an office app spawning a shell).

Proven working: a decoy named `soffice.bin` spawning `bash` triggered:
```
⚠️  SUSPICIOUS: soffice.bin -> bash
```

---

## Project structure

```
src/
├── main.rs      # orchestration: list /proc, filter PIDs, loop, print, run detection
├── process.rs   # reading facts about a process from /proc
└── detect.rs    # detection rules (is this relationship malicious?)
```

Each file = one Rust module. `main.rs` declares them with `mod process;` / `mod detect;`.
`pub` makes a function/struct visible to other files. Call with `process::read_name(...)`.

---

## Functions (each has ONE job)

| Function | File | Job | Reads |
|----------|------|-----|-------|
| `read_name(pid) -> Option<String>` | process.rs | PID -> process name | `/proc/<pid>/comm` |
| `read_stat(pid) -> Option<ProcInfo>` | process.rs | PID -> ppid + state | `/proc/<pid>/stat` |
| `is_suspicious(parent, child) -> bool` | detect.rs | yes/no: is this pair a red flag? | (pure logic) |

`ProcInfo` struct holds `ppid: u32` and `state: char`.

---

## Milestones

- [x] M1 — list numeric folders in `/proc` (the PIDs)
- [x] M2 — `read_name`: PID -> name from `/proc/<pid>/comm`
- [x] M3 — `read_stat`: parent PID + state from `/proc/<pid>/stat`
- [x] M4 — `is_suspicious`: flag office-app -> shell (first detection rule)
- [ ] M5 — fix remaining `main.rs` cleanups (count/double-read), add process owner (UID)
- [ ] M6 — search/filter by name; build a parent -> child tree
- [ ] M7 — more detection rules; load rules from a config file

---

## Key lessons learned

### Linux / systems
- `/proc` is a **virtual filesystem** — the kernel invents these files live on each read; nothing is on disk. Proven by watching `/proc/uptime` change between reads.
- `/proc/<pid>/comm` = name, `/proc/<pid>/stat` = ppid + state + more.
- Reading `/proc` is **I/O-bound** (syscalls). That's why Rust was only ~3x faster than
  Python here — both wait on the same kernel calls. Language speed only matters for the
  work the language itself does (CPU work), not syscalls.

### Security
- **Process enumeration** is the base of both offense (malware checking for AV/analysis
  tools) and defense (EDR spotting anomalies).
- **Direction matters in detection:** `word -> powershell` is an attack; `powershell -> word`
  is normal. Same names, opposite meaning. (This was a real bug I hit and fixed.)
- Tested the rule safely with a harmless decoy process, never real malware.

### Rust
- `Option` (Some/None) and `Result` (Ok/Err); `.ok()` turns Result into Option.
- `?` = early-return on failure. `match` and `if let` for handling them.
- `parse::<u32>()` = text -> number; used to filter real PIDs from `/proc`.
- Defensive parsing: `/proc/<pid>/stat` names are in `(parens)` and can contain spaces/`)`,
  so parse AFTER the last `)` (`rfind(')')`) to resist hostile process names.
- Modules: `mod`, `pub`, and the `::` path. Split code by job across files.
- The `_` underscore has 3 uses: word separator (`read_name`), ignore a value (`Err(_)`),
  mute unused warning (`let _x`).
- Timing with `std::time::Instant` + `Duration`; always benchmark with `--release`.

---

## Known cleanups still open in main.rs
- Earlier versions: `count` not incremented; `read_name`/`read_stat` sometimes called twice.
  (Current version wires detection correctly — revisit these in M5.)

---

## How to run

```bash
cargo run              # debug
cargo run --release    # optimized (use for timing)
```

## How to test detection safely
Create a harmless decoy that mimics "office app spawns shell":
```bash
cp /bin/bash /tmp/soffice.bin
setsid /tmp/soffice.bin -c 'bash -c "sleep 30; :" & sleep 30' &
cargo run | grep -i suspicious
pkill -f soffice.bin   # clean up
```
