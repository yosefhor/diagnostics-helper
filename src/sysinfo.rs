use sysinfo::{System, RefreshKind, CpuRefreshKind};

pub fn get_cpu_usage() -> Option<f32> {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
    );

    sys.refresh_cpu_all();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    if cpus.is_empty() {
        return None;
    }

    let usage = cpus[0].cpu_usage();
    Some(usage)
}

pub fn get_memory_usage() -> Option<(u64, u64)> {
    let mut sys = System::new_all();  

    sys.refresh_memory();            

    let used_kb = sys.used_memory();
    let total_kb = sys.total_memory();

    let used_mb = used_kb / 1024;
    let total_mb = total_kb / 1024;

    Some((used_mb, total_mb))
}
