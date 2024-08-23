use chrono;
use swayipc::{Connection, Fallible};
use sysinfo::{System, SystemExt, DiskExt, NetworkExt, NetworksExt, CpuExt};
use std::time::Duration;
use std::thread;
use std::io::{Read, Write};
use std::fs::File;
use std::process::Command;

fn get_amd_gpu_usage() -> Result<f64, std::io::Error> {
  let output = Command::new("/opt/rocm/bin/rocm-smi")
    .output()?;
  
  if output.status.success() {
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
      if line.starts_with("0") {  // Look for the first GPU (index 0)
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 15 {  // Ensure we have enough parts
          if let Ok(usage) = parts[14].trim().replace("%", "").parse::<f64>() {
            return Ok(usage);
          }
        }
      }
    }
  }
  Ok(0.0) 
}

fn main() -> Fallible<()> {
  let mut connection = Connection::new()?;
  connection.run_command("bar new_bar position top")?;
  connection.run_command("bar new_bar status_command ~/devel/statusbar/target/debug/statusbar")?;

  let mut sys = System::new_all();

  loop {
    sys.refresh_all();

    let main_disk = sys.disks().iter().find(|disk| disk.name() == "/dev/nvme0n1p3").unwrap();
    let other_disk = sys.disks().iter().find(|disk| disk.name() == "/dev/nvme1n1p1").unwrap();
    let main_usage = 100.0 - (main_disk.available_space() as f64 / main_disk.total_space() as f64 * 100.0);
    let other_usage = 100.0 - (other_disk.available_space() as f64 / other_disk.total_space() as f64 * 100.0);

    let mem_usage = sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0;
    let cpu_usage = sys.global_cpu_info().cpu_usage();

    // @TODO: GPU agnostic
    let gpu_usage = get_amd_gpu_usage().unwrap_or(0.0);

    let network_status = if sys.networks().iter().any(|(_, net)| net.received() > 0 || net.transmitted() > 0) {
      if sys.networks().iter().any(|(name, _)| name.starts_with("w")) {
        "\u{f1eb}" // Wireless
      } 
      else {
        "\u{f796}" // Ethernet
      }
    } 
    else {
      "\u{f818}" // Disconnected
    };

    let time = chrono::Local::now();
    let status = format!(
      "Disk1: {:.1}% | Disk2: {:.1}% | Mem: {:.1}% | CPU: {:.1}% | GPU: {:.1}% | {}",
      main_usage, other_usage, mem_usage, cpu_usage, gpu_usage,
      time.format("%F %I:%M:%S %p")
    );

    println!("{}", status);

    std::io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs_f64(1.0 / 60.0));
  }
}

