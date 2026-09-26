mod toml_parser;
mod utils;

use libc;
use proc_parser::{Parsers, ProcFilePIDStat};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::time::Duration;
use toml_parser::PROC_ROOT_PATH;
use utils::signal_handler;

fn main() -> Result<(), Box<dyn Error>> {
    let is_root = unsafe { libc::getuid() == 0 };
    println!("is_root: {}", is_root);

    let signal_hook = signal_handler();
    let proc_path = Path::new(PROC_ROOT_PATH);
    let mut pid_map: HashMap<u32, ProcFilePIDStat> = HashMap::new();
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
        let pid_path = proc_path.join(pid_val.to_string()).join("status");
        let cmdline = ProcFilePIDStat::new(&pid_path.to_str().unwrap());
        if cmdline.is_none() {
            continue;
        }
        pid_map.insert(pid_val, cmdline.unwrap());
    }

    // form the keys array from pid_map keys
    let keys: Vec<u32> = pid_map.keys().cloned().collect();
    while !signal_hook.load(Ordering::Relaxed) {
        for key in keys.iter() {
            if *key == 1781 {
                let cmdline = pid_map.get_mut(&key).unwrap();
                let mut lines: [usize; 5] = [0; 5];
                let mut spaces: [usize; 40] = [0; 40];
                cmdline.read();
                // let parens = cmdline.parse_string(5, 19);
                cmdline.parse_newlines(&mut lines);
                cmdline.parse_spaces(&mut spaces);
                println!("Spaces: {:?} ,lines: {:?}", spaces, lines);
                let data = cmdline.parse_bytes();
                println!("data: {:?}", data);
                // println!("{:?}", cmdline.buffer);
            }
        }
        // break;
        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
