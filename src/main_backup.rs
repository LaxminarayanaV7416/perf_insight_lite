mod constants;
mod proc_collector {
    pub mod pid_tree;
    pub mod proc_file_reader;
}
mod utils;

use constants::PROC_ROOT_PATH;
use proc_collector::pid_tree::PidNode;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::atomic::Ordering;
use utils::signal_handler;

fn get_ppid(pid: u32, proc_path: &Path) -> u32 {
    let status_path = proc_path.join(pid.to_string()).join("status");
    let status = fs::read_to_string(status_path).unwrap_or_default();
    let ppid = status
        .lines()
        .find(|l| l.starts_with("PPid:"))
        .map(|l| l.split_whitespace().nth(1).unwrap_or_default())
        .and_then(|g| g.parse::<u32>().ok())
        .unwrap_or_default();
    ppid
}

fn get_pid_command(pid: u32, proc_path: &Path) -> String {
    let cmdline_path = proc_path.join(pid.to_string()).join("cmdline");
    let cmdline = fs::read_to_string(cmdline_path).unwrap_or_default();
    cmdline.replace('\0', " ")
}

fn get_pid_comm(pid: u32, proc_path: &Path) -> String {
    let cmdline_path = proc_path.join(pid.to_string()).join("comm");
    let cmdline = fs::read_to_string(cmdline_path).unwrap_or_default();
    cmdline.trim().to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let signal_hook = signal_handler();
    let proc_path = Path::new(PROC_ROOT_PATH);

    // assume that we are having a ppid 0 from the start
    let ppid_child: Vec<PidNode> = Vec::new();
    let mut root = PidNode::new(0, "".to_string(), "".to_string(), ppid_child);
    while !signal_hook.load(Ordering::Relaxed) {
        for entry in fs::read_dir(proc_path).into_iter().flatten().flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let pid: Option<u32> = path
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(|s| s.parse::<u32>().ok());
            let pid_val = match pid {
                Some(v) => v,
                None => continue,
            };
            let ppid = get_ppid(pid_val, &proc_path);
            let cmdline = get_pid_command(pid_val, &proc_path);
            let comm = get_pid_comm(pid_val, &proc_path);
            let children: Vec<PidNode> = Vec::new();

            if root.find(ppid) {
                root.find_and_add_child(ppid, pid_val, &comm, &cmdline, &children);
            }
        }
    }
    root.print(None, 0);
    // root.remove_child(2);
    // root.print(None, 0);

    Ok(())
}
