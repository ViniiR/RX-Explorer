use std::{
    fs::{self, create_dir, remove_dir_all, remove_file, File, OpenOptions},
    io::{Read, Write}, path::Path
};
use tokio::sync::mpsc;
use tokio::task;
use serde::Serialize;
use sysinfo::Disks;
use walkdir::WalkDir;
use dirs;
/// Drive is a structure to represent windows Hard disks 
/// it renames its properties to camelCase when sent to Javascript
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

///open_dir is a tauri command that takes a String argument which represents the directory to be open
///and returns a String Vector representing all the files inside a directory
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

///open_disk opens the provided windows hard drive
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

///display_disks displays all mounted windows hard drives and returns them to JavaScript
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

///open_with_app opens the provided file path with its default program set up on the user's system
#[tauri::command]
pub async fn open_with_app(path: String) {
    let _ = open::commands(path)[0].status();
}

///search_file does a recursive, multi threaded search on the provided path, it only searches files and not directories
#[tauri::command]
pub async fn search_file(path: &str, file: &str) -> Result<Vec<String>, String> {
    let (tx, mut rx) = mpsc::channel(100);

    for entry in WalkDir::new(path) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue, // Skip files/directories that can't be read
        };

        let tx = tx.clone();
        let file = file.to_string();

        task::spawn(async move {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.contains(&file) {
                    if let Some(path) = entry.path().to_str() {
                        tx.send(path.to_string()).await.unwrap();
                    }
                }
            }
        });
    }

    drop(tx); // Close the channel

    let mut results = Vec::new();

    while let Some(result) = rx.recv().await {
        results.push(result);
    }

    if results.is_empty() {
        Err("No matching files found".to_string())
    } else {
        Ok(results) // No need to clone the results
    }
}

///reads the provided file path and returns a Result Ok raw string of all the file contents, returns a err string if it is unable to open the provided file,
/// it may fail and return an error if such file requires super user privileges to be open and the app is not running on super user
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

///retrieves the user's current user home directory and downloads, documents and pictures
#[tauri::command]
pub async fn get_favorites() -> Vec<String> {
    let home = dirs::home_dir();
    let favorites = vec![
        "Downloads", "Documents", "Pictures"
    ];
    dbg!(&home);
    let mut directories = vec![home.clone().unwrap().to_str().unwrap().to_string()];
    for item in favorites {
        let home = home.as_ref().unwrap().to_str().unwrap();
        directories.push(format!("{}\\{}", home, item))
    }
    directories
}

///writes the provided string into the provided file path, no matter if the provided file already has content written in, it will rewrite over it
/// if you are editing the file this is no big deal
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

///creates a file with provided name on provided path if it already doesn't exist
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

///creates a directory with provided name on provided path only if it already doesn't exist
///if it does, it returns an err
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

///checks using the std::fs crate if the file on provided path is a directory or not
#[tauri::command]
pub async fn is_dir(file: &str) -> Result<bool, bool> {
    if let Ok(data) = fs::metadata(file) {
        if data.is_dir() {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Err(false)
    }
}

///deletes the file or directory on provided path, if it has content inside it, it will do a recursive deletion on all nested files
///it does not send files to trash can, they will be permanently deleted
/// if the provided directory or path requires super user privileges without having so, it will return false
#[tauri::command]
pub async fn delete_file(file_path: String) -> bool {
    if is_dir(&file_path).await.expect("err") {
        match remove_dir_all(&file_path) {
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