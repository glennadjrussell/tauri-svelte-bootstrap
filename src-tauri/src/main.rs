// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;

#[tauri::command]
fn discover_network() {
    let tcp_file = Path::new("/proc/net/tcp");
    let data = match fs::read_to_string(tcp_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading /proc/net/tcp: {}", err);
            return;
        }
    };

    for line in data.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 11 {
            let local_address = parts[1];
            let remote_address = parts[2];
            let inode = parts[9];

            println!("Local: {}, Remote: {}, Inode: {}", local_address, remote_address, inode);
            // You can use the inode to find more information about the connection, e.g., in /proc/<pid>/fd/
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
