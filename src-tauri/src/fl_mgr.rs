use std::{env, fs::{self, create_dir, remove_dir, remove_file, File, OpenOptions},
    io::{Read, Write}, path::Path};
use serde::Serialize;
use sysinfo::Disks;
use rayon::prelude::*;
use walkdir::WalkDir;
use std::sync::{Arc, Mutex};

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
    usage_percentage: String,
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
pub fn open_disk(disk_name: String) -> Vec<String> {
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
    let usage_percentage = format!(
        "{:.2}",
        ((1.0 - available_space / total_space) * 100.0).to_string()
    );
    Drive {
        name: String::from("C"),
        available_capacity: disk.available_space().to_string(),
        total_capacity: disk.total_space().to_string(),
        file_system: disk.file_system().to_str().expect("err").to_string(),
        disk_type: disk.kind().to_string(),
        usage_percentage,
    }
}

#[tauri::command]
pub async fn open_with_app(path: String) {
    let _ = open::commands(path)[0].status();
}

#[tauri::command]
pub async fn search_file(path: &str, file: &str) -> Result<Vec<String>, String> {
    let results = Arc::new(Mutex::new(vec![])); // Concurrent vector

    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .par_bridge() // Parallelize with Rayon
        .for_each(|entry| {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.contains(file) {
                    if let Some(path) = entry.path().to_str() {
                        let mut results = results.lock().unwrap(); // Lock the mutex
                        results.push(path.to_string());
                    }
                }
            }
        });

    let results = results.lock().unwrap(); // Lock the mutex to get the results

    if results.is_empty() {
        Err("No matching files found".to_string())
    } else {
        Ok(results.clone()) // Return a clone of the results
    }
}

#[tauri::command]
pub async fn read_file(file_name: String) -> Result<String, String> {
    let file = file_name;
    println!("{:?}", file);
    let mut file = match File::open(file) {
        Ok(text) => {
            text
        },
        Err(_) => {
            return Err("Error opening file".to_string())
        },
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            print!("File read successfully")
        },
        Err(_) => {
            return Err(String::from("Error opening file, no permission"))
        }
    }

    Ok(contents.to_string())
}

#[tauri::command]
pub async fn get_favorites() -> Vec<String> {
    let home = env::var_os("HOME");
    let favorites = vec![
        "Downloads", "Documents", "Pictures"
    ];
    let mut directories = vec![home.clone().unwrap().to_str().unwrap().to_string()];
    for item in favorites {
        let home = home.as_ref().unwrap().to_str().unwrap();
        directories.push(format!("{}\\{}", home, item))
    }
    directories
}

#[tauri::command]
pub async fn save_file(file: String, text: String) {
    let mut file = File::create(file).expect("error finding file");

    match file.write_all(text.as_bytes()) {
        Ok(_) => {
            print!("File saved successfully");
        },
        Err(_) => {
            print!("Error writing file");
        },
    }
}


#[tauri::command]
pub async fn create_file(file_name: String, file_location: String) -> String {
    let file_full_path = format!("{file_location}\\{file_name}");
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&file_full_path);

    match file {
        Ok(_) => {
            print!("file create successfully");
        },
        Err(_) => {
            eprint!("failed to create file");
        }
    }
    file_full_path
}

#[tauri::command]
pub async fn create_directory(dir_name: String, dir_location: String)
-> Result<String, String> 
{
    let path = format!("{dir_location}\\{dir_name}");
    let path = Path::new(&path);

    match create_dir(path) {
        Ok(_) => {
            Ok("Directory created successfully".to_string())
        },
        Err(_) => {
            Err("Failed to create directory".to_string())
        }
    }
}

#[tauri::command]
pub async fn is_dir(file: String) -> bool {
    if let Ok(data) = fs::metadata(file) {
        if data.is_dir() {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

#[tauri::command]
pub async fn delete_file(file_path: String) -> bool{
    if is_dir(file_path.clone()).await {
        match remove_dir(&file_path) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
    } else {
        match remove_file(&file_path) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
    }
}