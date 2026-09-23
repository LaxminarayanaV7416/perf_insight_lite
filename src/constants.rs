// use std::path::Path;

// we will use this path to read process information
// from the procFS which is a virtual filesystem in linux
//

pub const PROC_ROOT_PATH: &str = "/proc";
// pub const SYS_ROOT_PATH: &str = "/sys";

// // /proc/stat (CPU utilization and attributes)
// pub const PROC_STAT_PATH: &Path = &PROC_ROOT_PATH.join("stat");
// // /proc/meminfo (memory information)
// pub const PROC_MEMINFO_PATH: &Path = &PROC_ROOT_PATH.join("meminfo");
// // /proc/vmstat (system performance)
// pub const PROC_VMSTAT_PATH: &Path = &PROC_ROOT_PATH.join("vmstat");
// // /sys/fs/cgroup (Control Groups - Linux Containers)
// pub const SYS_FS_CGROUP_PATH: &Path = SYS_ROOT_PATH.join("fs/cgroup");
// // /proc/interrupts (total and per core hardware interrupts)
// pub const PROC_INTERRUPTS_PATH: &Path = PROC_ROOT_PATH.join("interrupts");
// // /proc/softirqs (total and per core software interrupts)
// pub const PROC_SOFTIRQS_PATH: &Path = PROC_ROOT_PATH.join("softirqs");
// // /proc/loadavg (system load and total processes running)
// pub const PROC_LOADAVG_PATH: &Path = PROC_ROOT_PATH.join("loadavg");
// // /proc/pressure/{cpu,memory,io} (pressure stall information)
// pub const PROC_PRESSURE_PATH: &Path = PROC_ROOT_PATH.join("pressure");
// pub const PROC_PRESSURE_CPU_PATH: &Path = PROC_PRESSURE_PATH.join("cpu");
// pub const PROC_PRESSURE_MEMORY_PATH: &Path = PROC_PRESSURE_PATH.join("memory");
// pub const PROC_PRESSURE_IO_PATH: &Path = PROC_PRESSURE_PATH.join("io");
