use std::thread;
use std::time::Duration;

use sysinfo::System;
pub use systemstat::platform::{Platform, PlatformImpl as SystemS};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SysStats {
    #[serde(rename = "cpuUsage")]
    pub cpu_usage: String,
    #[serde(rename = "ramUsage")]
    pub ram_usage: String,
    #[serde(rename = "swapUsage")]
    pub swp_usage: String,
}

#[tauri::command]
pub fn start_tsk_mgr() -> SysStats {
    let mut sys = System::new_all();
    let sys_s = SystemS::new();
    sys.refresh_all();

    SysStats {
        ram_usage: get_ram_usage(&sys),
        swp_usage: get_swp_usage(&sys),
        cpu_usage: get_cpu_percentage(&sys_s),
    }
}

fn get_swp_usage(sys: &System) -> String {
    let total_swp: f64 = sys.total_swap() as f64;
    let used_swp: f64 = sys.used_swap() as f64;
    let swp_percentage: f64 = (used_swp / total_swp) * 100.0;
    let swp_percentage = format!("{:.2}", swp_percentage);
    swp_percentage.to_string()
}

fn get_ram_usage(sys: &System) -> String {
    let total_ram: f64 = sys.total_memory() as f64;
    let used_ram: f64 = sys.used_memory() as f64;
    let ram_percentage: f64 = (used_ram / total_ram) * 100.0;
    let ram_percentage = format!("{:.2}", ram_percentage);
    ram_percentage.to_string()
}

fn get_cpu_percentage(sys: &SystemS) -> String {
    let mut cpu_usage = String::new();
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            cpu_usage = format!("{}", cpu.idle * 100.0);
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }
    cpu_usage
}
