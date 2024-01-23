use std::{fs::{self, DirEntry}, path::Display};

use serde::Serialize;
use sysinfo::Disks;

#[derive(Debug, Serialize)]
pub struct Drive {
    name: String,
    #[serde(rename = "availableCapacity")]
    available_capacity: String,
    #[serde(rename = "totalCapacity")]
    total_capacity: String,
    #[serde(rename = "fileSystem")]
    file_system: String,
    #[serde(rename = "diskType")]
    disk_type: String,
    #[serde(rename = "usagePercentage")]
    usage_percentage: String
}

#[tauri::command]
pub fn open_dir(dir: String) -> Vec<String> {
    let mut directories: Vec<String> = Vec::new();
    let mut dir = dir;
    if dir == "C:" {
        dir = "C:\\".to_string();
    }

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file = entry.path();
                    let file = file.display().to_string();
                    directories.push(file);
                }
            }
        }
        Err(_) => {
            eprintln!("Error: Denied access, requires administrator privileges");
        }
    }
    directories
}

#[tauri::command]
pub fn open_disk(disk_name: String) -> Vec<String>{
    let c_drive_path = format!("{disk_name}:\\");
    let mut directories: Vec<String> = Vec::new();

    match fs::read_dir(c_drive_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file = entry.path();
                    let file = file.display().to_string();
                    directories.push(file);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading C drive: {}", e);
        }
    }
    directories
}

#[tauri::command]
pub fn display_disks() -> Drive {
    let disks = Disks::new_with_refreshed_list();
    let disk = disks.list().get(0).unwrap();
    let total_space = disk.total_space() as f64;
    let available_space = disk.available_space() as f64;
    let usage_percentage = format!("{:.2}",((1.0 - available_space / total_space) * 100.0).to_string());
    Drive {
        name: String::from("C"),
        available_capacity: disk.available_space().to_string(),
        total_capacity: disk.total_space().to_string(),
        file_system: disk.file_system().to_str().expect("err").to_string(),
        disk_type: disk.kind().to_string(),
        usage_percentage
    }
}