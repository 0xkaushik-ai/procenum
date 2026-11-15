mod process;

use std::fs;
use std::time::Instant;

mod detect;

fn main () {
    let start = Instant::now();
    let mut count = 0;

let entries = fs::read_dir("/proc").expect("Cant read /proc");

for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();

        let pid = match name.parse::<u32>() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let name = match process::read_name(pid) {
            Some(n) => n,
            None => continue,
        };
        let info = match process::read_stat(pid) {
            Some(i) => i,
            None => continue,
        };

        println!(
            "PID: {pid:>6}  PPID: {ppid:>6}  [{state}]  {name}",
            ppid = info.ppid,
            state = info.state,
        );
        count += 1;

        // Detection: look up the PARENT's name, then test the rule.
        if let Some(parent_name) = process::read_name(info.ppid) {
            if detect::is_suspicious(&parent_name, &name) {
                println!("   ⚠️  SUSPICIOUS: {parent_name} -> {name}");
            }
        }
    }

    println!("\nTotal processes found: {count}");
    println!("Scan time: {:?}", start.elapsed());
}