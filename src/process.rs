
pub struct ProcInfo {
    pub ppid: u32,    // parent process ID — who launched it
    pub state: char,  // R=running, S=sleeping, Z=zombie, ...
}

/// Job: turn a PID into that process's name. Returns None if the process is gone.
pub fn read_name(pid: u32) -> Option<String> {
    let path = format!("/proc/{pid}/comm");
    let name = std::fs::read_to_string(path).ok()?;
    Some(name.trim().to_string())
}

/// Job: read parent PID + state from /proc/<pid>/stat.
pub fn read_stat(pid: u32) -> Option<ProcInfo> {
    let path = format!("/proc/{pid}/stat");
    let content = std::fs::read_to_string(path).ok()?;

    let after_name = &content[content.rfind(')')? + 1..];
    let fields: Vec<&str> = after_name.split_whitespace().collect();

    let state = fields.first()?.chars().next()?;   // 1st field = state
    let ppid: u32 = fields.get(1)?.parse().ok()?;  // 2nd field = ppid

    Some(ProcInfo { ppid, state })
}
