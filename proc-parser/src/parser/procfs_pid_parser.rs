use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
// lib imports
use crate::common::procfs_constants::PROC_FS_ROOT_PATH;
use crate::configs::config::{Config, ProcFSConfigTrait};
use crate::procfs::pidfs::cachable_reads::{
    parse_procfs_pid_cgroup, parse_procfs_pid_cmdline, parse_procfs_pid_comm,
};

pub fn record_procfs_pid(pid: usize, config: &Config, signaller: Arc<AtomicBool>) {
    let pid_comm = parse_procfs_pid_comm(pid);
    let pid_cmdline = parse_procfs_pid_cmdline(pid);
    let pid_cgroup = parse_procfs_pid_cgroup(pid);
    // TODO here after implementation we will dispatch the cgroup watcher
    
}
