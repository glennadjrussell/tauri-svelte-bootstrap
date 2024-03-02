// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod network;

use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
struct NetworkConnection {
    local_ip: String,
    local_port: u8,
    remote_ip: String,
    remote_port: u8
}

#[derive(serde::Serialize)]
struct NetworkState {
    connections: Vec<NetworkConnection>
}

#[tauri::command]
fn discover_network() -> NetworkState {
    let tcp_file = Path::new("/proc/net/tcp");
    let data = match fs::read_to_string(tcp_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading /proc/net/tcp: {}", err);
            return;
        }
    };

    let conns = Vec<NetworkConnection>::default();

    for line in data.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 11 {
            let local_address = parts[1];
            let remote_address = parts[2];
            let inode = parts[9];

            println!("Local: {}, Remote: {}, Inode: {}", local_address, remote_address, inode);

            if let Ok(parsed_addr) = network::convert_proc_net_tcp_address(&local_address) {
                println!("Parsed IP: {}, Parsed Port: {}", parsed_addr.0, parsed_addr.1);
            }

            if let Ok(parsed_remote) = network::convert_proc_net_tcp_address(&remote_address) {
                println!("Remote IP: {}, Remote Port: {}", parsed_remote.0, parsed_remote.1);
            }
            // You can use the inode to find more information about the connection, e.g., in /proc/<pid>/fd/
        }
    }

    NetworkState {
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, discover_network])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
