use std::thread;
use std::time::Duration;
use sysinfo::System;
pub use systemstat::platform::{Platform, PlatformImpl as SystemS};
use serde::Serialize;

///a struct to organize system status information returned by the main function on this file
#[derive(Debug, Serialize)]
pub struct SysStats {
    #[serde(rename = "cpuUsage")]
    pub cpu_usage: String,
    #[serde(rename = "ramUsage")]
    pub ram_usage: String,
    #[serde(rename = "swapUsage")]
    pub swp_usage: String,
    #[serde(rename = "ramNumbers")]
    pub ram_numbers: Vec<f64>,
    #[serde(rename = "swpNumbers")]
    pub swp_numbers: Vec<f64>,
}

///starts the task manager sub-app and is the main entrance for this file
#[tauri::command]
pub async fn start_tsk_mgr() -> SysStats {
    let mut sys = System::new_all();
    let sys_s = SystemS::new();
    sys.refresh_all();

    SysStats {
        cpu_usage: get_cpu_percentage(&sys_s).await,
        ram_usage: get_ram_usage(&sys, false).await.unwrap(),
        swp_usage: get_swp_usage(&sys, false).await.unwrap(),
        ram_numbers: get_ram_usage(&sys, true).await.unwrap_err(),
        swp_numbers: get_swp_usage(&sys, true).await.unwrap_err(),
    }
}

///returns the system swap usage, also know as virtual ram, in bytes or in its percentage usage based on the bytes boolean var
async fn get_swp_usage(sys: &System, bytes: bool) -> Result<String, Vec<f64>> {
    let total_swp: f64 = sys.total_swap() as f64;
    let used_swp: f64 = sys.used_swap() as f64;
    let swp_percentage: f64 = (used_swp / total_swp) * 100.0;
    let swp_percentage = format!("{:.2}", swp_percentage);
    if bytes {
        Err(Vec::from([used_swp, total_swp]))
    } else {
        Ok(swp_percentage)
    }
}

///returns the current ram usage in total bytes or in percentage based on the bytes boolean var
async fn get_ram_usage(sys: &System, bytes: bool) -> Result<String, Vec<f64>> {
    let total_ram: f64 = sys.total_memory() as f64;
    let used_ram: f64 = sys.used_memory() as f64;
    let ram_percentage: f64 = (used_ram / total_ram) * 100.0;
    let ram_percentage = format!("{:.2}", ram_percentage);
    if bytes {
        Err(Vec::from([used_ram, total_ram]))
    } else {
        Ok(ram_percentage)
    }
}

///returns the current cpu usage percentage eg. 21.98
async fn get_cpu_percentage(sys: &SystemS) -> String {
    let mut cpu_usage = String::new();
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            let total_usage = cpu.user + cpu.nice + cpu.system;
            cpu_usage = format!("{:.2}", total_usage * 100.0);
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }
    cpu_usage
}
