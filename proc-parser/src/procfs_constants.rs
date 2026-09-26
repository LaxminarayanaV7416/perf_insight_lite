pub const PROC_FS_ROOT_PATH: &str = "/proc";
pub const SYS_FS_ROOT_PATH: &str = "/sys";

// PROC folder constants
// The folder is not important in metrics collection
// =================================================
// pub const PROC_ACPI_FOLDER_SLUG: &str = "acpi";
// pub const PROC_ASOUND_FOLDER_SLUG: &str = "asound";
// pub const PROC_BUS_FOLDER_SLUG: &str = "bus";
// pub const PROC_DRIVER_FOLDER_SLUG: &str = "driver";
// pub const PROC_DYNAMIC_DEBUG_FOLDER_SLUG: &str = "dynamic_debug";
// // The above folder contains only config file which is used to debug
// // running kernel, not important for metrics collection but we can
// // still collect it for debugging purposes; enable in future
// pub const PROC_FS_FOLDER_SLUG: &str = "fs";
// pub const PROC_IRQ_FOLDER_SLUG: &str = "irq";
pub const PROC_PRESSURE_FOLDER_SLUG: &str = "pressure";
pub const PROC_SCSI_FOLDER_SLUG: &str = "scsi";
pub const PROC_SYS_FOLDER_SLUG: &str = "sys";
pub const PROC_SYSVIPC_FOLDER_SLUG: &str = "sysvipc";
pub const PROC_TTY_FOLDER_SLUG: &str = "tty";
// pub const PROC_NET_FOLDER_SLUG: &str = "net";
// pub const PROC_THREAD_SELF_FILE_SLUG: &str = "thread-self";
// pub const PROC_SELF_FILE_SLUG: &str = "self";

// PROC FOLDER pressure files
pub const PROC_PRESSURE_CPU_FILE_SLUG: &str = "cpu";
pub const PROC_PRESSURE_MEMORY_FILE_SLUG: &str = "memory";
pub const PROC_PRESSURE_IO_FILE_SLUG: &str = "io";
pub const PROC_PRESSURE_IRQ_FILE_SLUG: &str = "irq";

// PROC file constants
pub const PROC_BOOTCONFIG_FILE_SLUG: &str = "bootconfig";
pub const PROC_BUDDYINFO_FILE_SLUG: &str = "buddyinfo";
pub const PROC_CGROUPS_FILE_SLUG: &str = "cgroups";
pub const PROC_CMDLINE_FILE_SLUG: &str = "cmdline";
pub const PROC_CONSOLES_FILE_SLUG: &str = "consoles";
pub const PROC_CPUINFO_FILE_SLUG: &str = "cpuinfo";
pub const PROC_CRYPTO_FILE_SLUG: &str = "crypto";
pub const PROC_DEVICES_FILE_SLUG: &str = "devices";
pub const PROC_DISKSTATS_FILE_SLUG: &str = "diskstats";
pub const PROC_DMA_FILE_SLUG: &str = "dma";
pub const PROC_EXECDOMAINS_FILE_SLUG: &str = "execdomains";
pub const PROC_FB_FILE_SLUG: &str = "fb";
pub const PROC_FILESYSTEMS_FILE_SLUG: &str = "filesystems";
pub const PROC_INTERRUPTS_FILE_SLUG: &str = "interrupts";
pub const PROC_IOMEM_FILE_SLUG: &str = "iomem";
pub const PROC_IOPORTS_FILE_SLUG: &str = "ioports";
pub const PROC_KALLSYMS_FILE_SLUG: &str = "kallsyms";
pub const PROC_KCORE_FILE_SLUG: &str = "kcore";
pub const PROC_USERS_FILE_SLUG: &str = "key-users";
pub const PROC_KEYS_FILE_SLUG: &str = "keys";
pub const PROC_KMSG_FILE_SLUG: &str = "kmsg";
pub const PROC_KPAGECGROUP_FILE_SLUG: &str = "kpagecgroup";
pub const PROC_KPAGECOUNT_FILE_SLUG: &str = "kpagecount";
pub const PROC_KPAGEFLAGS_FILE_SLUG: &str = "kpageflags";
pub const PROC_LOADAVG_FILE_SLUG: &str = "loadavg";
pub const PROC_LOCKS_FILE_SLUG: &str = "locks";
pub const PROC_MEMINFO_FILE_SLUG: &str = "meminfo";
pub const PROC_MISC_FILE_SLUG: &str = "misc";
pub const PROC_MODULES_FILE_SLUG: &str = "modules";
pub const PROC_MTD_FILE_SLUG: &str = "mtd";
pub const PROC_MTRR_FILE_SLUG: &str = "mtrr";
pub const PROC_PAGETYPEINFO_FILE_SLUG: &str = "pagetypeinfo";
pub const PROC_PARTITIONS_FILE_SLUG: &str = "partitions";
pub const PROC_SCHEDSTAT_FILE_SLUG: &str = "schedstat";
pub const PROC_SLABINFO_FILE_SLUG: &str = "slabinfo";
pub const PROC_SOFTIRQS_FILE_SLUG: &str = "softirqs";
pub const PROC_STAT_FILE_SLUG: &str = "stat";
pub const PROC_SWAPS_FILE_SLUG: &str = "swaps";
pub const PROC_TRIGGER_FILE_SLUG: &str = "sysrq-trigger";
pub const PROC_TIMER_LIST_FILE_SLUG: &str = "timer_list";
pub const PROC_UPTIME_FILE_SLUG: &str = "uptime";
pub const PROC_VERSION_FILE_SLUG: &str = "version";
pub const PROC_VMALLOCINFO_FILE_SLUG: &str = "vmallocinfo";
pub const PROC_VMSTAT_FILE_SLUG: &str = "vmstat";
pub const PROC_ZONEINFO_FILE_SLUG: &str = "zoneinfo";
// pub const PROC_CONFIG_GZ_FILE_SLUG: &str = "config.gz";
// pub const PROC_MOUNTS_FILE_SLUG: &str = "mounts";

// PROC PID FOLDER CONSTANTS
pub const PROC_PID_ATTR_FOLDER_SLUG: &str = "attr";
pub const PROC_PID_FD_FOLDER_SLUG: &str = "fd";
pub const PROC_PID_FDINFO_FOLDER_SLUG: &str = "fdinfo";
pub const PROC_PID_MAP_FILES_FOLDER_SLUG: &str = "map_files";
pub const PROC_PID_NET_FOLDER_SLUG: &str = "net";
pub const PROC_PID_NS_FOLDER_SLUG: &str = "ns";
pub const PROC_PID_TASK_FOLDER_SLUG: &str = "task";

// PROC PID file constans
pub const PROC_PID_ARCH_STATUS_FILE_SLUG: &str = "arch_status";
pub const PROC_PID_AUTOGROUP_FILE_SLUG: &str = "autogroup";
pub const PROC_PID_AUXV_FILE_SLUG: &str = "auxv";
pub const PROC_PID_CGROUP_FILE_SLUG: &str = "cgroup";
pub const PROC_PID_CLEAR_REFS_FILE_SLUG: &str = "clear_refs";
pub const PROC_PID_CMDLINE_FILE_SLUG: &str = "cmdline";
pub const PROC_PID_COMM_FILE_SLUG: &str = "comm";
pub const PROC_PID_COREDUMP_FILTER_FILE_SLUG: &str = "coredump_filter";
pub const PROC_PID_CPU_RECTRL_GROUPS_FILE_SLUG: &str = "cpu_resctrl_groups";
pub const PROC_PID_ENVIRON_FILE_SLUG: &str = "environ";
pub const PROC_PID_GID_MAP_FILE_SLUG: &str = "gid_map";
pub const PROC_PID_IO_FILE_SLUG: &str = "io";
pub const PROC_PID_KSM_MERGING_PAGES_FILE_SLUG: &str = "ksm_merging_pages";
pub const PROC_PID_KSM_STAT_FILE_SLUG: &str = "ksm_stat";
pub const PROC_PID_LIMITS_FILE_SLUG: &str = "limits";
pub const PROC_PID_LOGINUID_FILE_SLUG: &str = "loginuid";
pub const PROC_PID_MAPS_FILE_SLUG: &str = "maps";
pub const PROC_PID_MEM_FILE_SLUG: &str = "mem";
pub const PROC_PID_MOUNTINFO_FILE_SLUG: &str = "mountinfo";
pub const PROC_PID_MOUNTS_FILE_SLUG: &str = "mounts";
pub const PROC_PID_MOUNTSTATS_FILE_SLUG: &str = "mountstats";
pub const PROC_PID_NUMA_MAPS_FILE_SLUG: &str = "numa_maps";
pub const PROC_PID_OOM_ADJ_FILE_SLUG: &str = "oom_adj";
pub const PROC_PID_OOM_SCORE_FILE_SLUG: &str = "oom_score";
pub const PROC_PID_OOM_SCORE_ADJ_FILE_SLUG: &str = "oom_score_adj";
pub const PROC_PID_PAGEMAP_FILE_SLUG: &str = "pagemap";
pub const PROC_PID_PERSONALITY_FILE_SLUG: &str = "personality";
pub const PROC_PID_PROJID_MAP_FILE_SLUG: &str = "projid_map";
pub const PROC_PID_SCHED_FILE_SLUG: &str = "sched";
pub const PROC_PID_SCHEDSTAT_FILE_SLUG: &str = "schedstat";
pub const PROC_PID_SESSIONID_FILE_SLUG: &str = "sessionid";
pub const PROC_PID_SETGROUPS_FILE_SLUG: &str = "setgroups";
pub const PROC_PID_SMAPS_FILE_SLUG: &str = "smaps";
pub const PROC_PID_SMAPS_ROLLUP_FILE_SLUG: &str = "smaps_rollup";
pub const PROC_PID_STACK_FILE_SLUG: &str = "stack";
pub const PROC_PID_STAT_FILE_SLUG: &str = "stat";
pub const PROC_PID_STATM_FILE_SLUG: &str = "statm";
pub const PROC_PID_STATUS_FILE_SLUG: &str = "status";
pub const PROC_PID_SYSCALL_FILE_SLUG: &str = "syscall";
pub const PROC_PID_TIMENS_OFFSETS_FILE_SLUG: &str = "timens_offsets";
pub const PROC_PID_TIMERS_FILE_SLUG: &str = "timers";
pub const PROC_PID_TIMERSLACK_NS_FILE_SLUG: &str = "timerslack_ns";
pub const PROC_PID_UID_MAP_FILE_SLUG: &str = "uid_map";
pub const PROC_PID_WCHAN_FILE_SLUG: &str = "wchan";
// pub const PROC_PID_CWD_FILE_SLUG: &str = "cwd";
// pub const PROC_PID_EXE_FILE_SLUG: &str = "exe";
// pub const PROC_PID_ROOT_FILE_SLUG: &str = "root";

// FILES Buffer sizes of all the files
