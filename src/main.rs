use std::fs;

use std::time::{Duration, Instant};


 fn read_name(pid: u32) -> Option<String> {
        let path = format!("/proc/{pid}/comm");
        let name = std::fs::read_to_string(path).ok()?;
        Some(name.trim().to_string())
    }
fn main() {
    let start = Instant::now();
    let mut read_name_total = Duration::ZERO;

    let mut count =0;
    let entries = fs::read_dir("/proc").expect("Cant read /proc");
    // proc means  the process filesystem it is the virtual filesystem 
   
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        
        let pid = match name.parse::<u32>() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let t = Instant::now();
        let result = read_name(pid);
        read_name_total += t.elapsed();




        match read_name(pid) {
            Some(name)=> {
                println!("PID: {pid:?},Name: {name}");
            }
            None => continue,

            
        }
    }
    println!("Total processes found: {count}");
    println!("Time inside read_name: {:?}", read_name_total);
    println!("Total time taken: {:?}", start.elapsed());

}