use chrono;
use swayipc::{Connection, Fallible};
use sysinfo::{System, SystemExt, DiskExt, NetworkExt, NetworksExt, CpuExt};
use std::time::Duration;
use std::thread;
use std::io::{Read, Write};
use std::fs::File;
use std::process::Command;
use std::env;

fn get_amd_gpu_usage() -> Result<f64, std::io::Error> {
  let output = Command::new("/opt/rocm/bin/rocm-smi")
    .output()?;
  
  if output.status.success() {
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
      if line.starts_with("0") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 15 { 
          if let Ok(usage) = parts[14].trim().replace("%", "").parse::<f64>() {
            return Ok(usage);
          }
        }
      }
    }
  }
  Ok(0.0) 
}

fn get_network_status(sys: &System) -> (String, String) {
  let fa_ethernet = "\u{f796}";
  let fa_wifi = "\u{f1eb}";
  let fa_disconnected = "\u{f071}";

  let mut active_interfaces = Vec::new();
  let mut network_debug = String::new();

  for (interface_name, data) in sys.networks().iter() {
    let received = data.received();
    let transmitted = data.transmitted();
    
    let is_up = std::fs::read_to_string(format!("/sys/class/net/{}/operstate", interface_name))
      .map(|s| s.trim() == "up")
      .unwrap_or(false);

    if is_up {
      active_interfaces.push(interface_name);
      network_debug.push_str(&format!("{}(R:{},T:{}) ", interface_name, received, transmitted));
    }
  }

  let status_icon = if active_interfaces.iter().any(|&name| name.starts_with("e")) {
    fa_ethernet
  } 
  else if active_interfaces.iter().any(|&name| name.starts_with("w")) {
    fa_wifi
  } 
  else {
    fa_disconnected
  };

  if active_interfaces.is_empty() {
    network_debug.push_str("No active interfaces");
  }

  (status_icon.to_string(), network_debug)
}

fn create_bar() -> Fallible<()> {
  let mut connection = Connection::new()?;
  connection.run_command("bar new_bar position top")?;
  connection.run_command("bar new_bar status_command ~/.cargo/bin/statusbar --status")?;
  connection.run_command("bar new_bar font pango:Berkeley Mono 9, Font Awesome 6 Free Solid 9")?;
  Ok(())
}

fn run_status_loop() -> Fallible<()> {
  let mut sys = System::new_all();

  let fa_disk_root = "\u{f0a0}";
  let fa_disk_home = "\u{f015}"; 
  let fa_memory = "\u{f538}";
  let fa_cpu = "\u{f2db}";          
  let fa_gpu = "\u{f26c}";
  let fa_wifi = "\u{f1eb}";
  let fa_ethernet = "\u{f796}"; 
  let fa_disconnected = "\u{f071}";
  let fa_clock = "\u{f017}";

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

    let (network_status, network_debug) = get_network_status(&sys);
    let time = chrono::Local::now();

    let status = format!(
      "<span font_desc='Font Awesome 6 Free Solid'>{}</span> {:5.1}% | <span font_desc='Font Awesome 6 Free Solid'>{}</span> {:5.1}% | <span font_desc='Font Awesome 6 Free Solid'>{}</span> {:5.1}% | <span font_desc='Font Awesome 6 Free Solid'>{}</span> {:5.1}% | <span font_desc='Font Awesome 6 Free Solid'>{}</span> {:5.1}% | <span font_desc='Font Awesome 6 Free Solid'>{}</span> | <span font_desc='Font Awesome 6 Free Solid'>{}</span> {}",
      fa_disk_home, main_usage,
      fa_disk_root, other_usage,
      fa_memory, mem_usage,
      fa_cpu, cpu_usage,
      fa_gpu, gpu_usage,
      network_status,
      fa_clock, time.format("%a %d %b %I:%M:%S %p")
    );
    println!("{}", status);

    std::io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs_f64(1.0 / 60.0));
  }
}

fn main() -> Fallible<()> {
  let args: Vec<String> = env::args().collect();
    
  if args.len() > 1 && args[1] == "--status" {
    run_status_loop()?;
  } 
  else {
    create_bar()?;
  }
  
  Ok(())
}

