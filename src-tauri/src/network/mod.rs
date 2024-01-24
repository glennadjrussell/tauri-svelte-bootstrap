#[derive(serde::Serialize)]
struct ProcNetTcpEntry {
    sl: usize,
    local_address: String,
    rem_address: String,
    st: String,
    tx_rx_queue: String,
    tr_tm_when: String,
    retrnsmt: String,
    uid: usize,
    timeout: usize,
    inode: usize,
}

fn convert_proc_net_tcp_address(hex_address: &str) -> Result<(String, u16), std::num::ParseIntError> {
    let parts: Vec<&str> = hex_address.split(":").collect();

    if parts.len() != 2 {
        return Err(std::num::ParseIntError::new(std::num::IntErrorKind::InvalidDigit, "Invalid address format"));
    }

    let ip_hex = parts[0];
    let port_hex = parts[1];

    let ip_num = u32::from_str_radix(ip_hex, 16)?;
    let ip_bytes = ip_num.to_be_bytes();
    let ip_addr = format!("", ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);

    let port_num = u16::from_str_radix(port_hex, 16)?;

    Ok((ip_addr, port_num));
}

fn convert_proc_net_entry(proc_entry: &str) -> Result<ProcNetTcpEntry, std::num::ParseIntError> {
}

