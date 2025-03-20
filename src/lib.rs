use std::io::{self, Error, ErrorKind};
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::Command as ProcessCommand;

/// Checks if a port is available.
///
/// Returns true if the port is available (not in use), false otherwise.
/// If verbose is true, prints information about the port state.
pub fn port_available(port: u16, verbose: bool) -> bool {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    match TcpListener::bind(socket_addr) {
        Ok(_) => {
            if verbose {
                println!("Port {} is available", port);
            }
            true
        }
        Err(_) => {
            if verbose {
                println!("Port {} is in use", port);
            }
            false
        }
    }
}

/// Kills the process using the given port.
///
/// Returns Ok(()) if successful, or an error if the process could not be killed.
/// If verbose is true, prints information about the process.
pub fn kill_port_process(port: u16, verbose: bool) {
    #[cfg(target_os = "windows")]
    let result = kill_port_process_windows(port, verbose);

    #[cfg(not(target_os = "windows"))]
    let result = kill_port_process_unix(port, verbose);

    if let Err(e) = result {
        if verbose {
            eprintln!("Failed to kill process on port {}: {}", port, e);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn kill_port_process_unix(port: u16, verbose: bool) -> io::Result<()> {
    // Find PID using lsof
    let lsof_output = ProcessCommand::new("lsof")
        .args(["-i", &format!(":{}", port), "-t"])
        .output()?;

    if !lsof_output.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to execute lsof command",
        ));
    }

    let pid_str = String::from_utf8_lossy(&lsof_output.stdout)
        .trim()
        .to_string();

    if pid_str.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("No process found using port {}", port),
        ));
    }

    // Kill process
    let kill_output = ProcessCommand::new("kill")
        .arg("-9")
        .arg(&pid_str)
        .output()?;

    if !kill_output.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Failed to kill process with PID {}", pid_str),
        ));
    }

    if verbose {
        println!(
            "Successfully killed process {} using port {}",
            pid_str, port
        );
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn kill_port_process_windows(port: u16, verbose: bool) -> io::Result<()> {
    // Find process using port with netstat
    let netstat_output = ProcessCommand::new("netstat")
        .args(["-ano", "|", "findstr", &format!(":{}", port)])
        .output()?;

    if !netstat_output.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to execute netstat command",
        ));
    }

    let output = String::from_utf8_lossy(&netstat_output.stdout);
    let lines: Vec<&str> = output.lines().collect();

    if lines.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("No process found using port {}", port),
        ));
    }

    // Extract PID from netstat output
    let last_line = lines[0];
    let parts: Vec<&str> = last_line.split_whitespace().collect();

    if parts.len() < 5 {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to parse netstat output",
        ));
    }

    let pid_str = parts[4];

    // Kill process
    let taskkill_output = ProcessCommand::new("taskkill")
        .args(["/F", "/PID", pid_str])
        .output()?;

    if !taskkill_output.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Failed to kill process with PID {}", pid_str),
        ));
    }

    if verbose {
        println!(
            "Successfully killed process {} using port {}",
            pid_str, port
        );
    }

    Ok(())
}
