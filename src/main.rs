use chrono;
use swayipc::{Connection, Fallible};
use std::time::Duration;
use std::thread;
use std::io::Write;

fn main() -> Fallible<()> {
  let mut connection = Connection::new()?;
  connection.run_command("bar new_bar position top")?;
  connection.run_command("bar new_bar status_command ~/devel/statusbar/target/debug/statusbar")?;

  loop {
    let time = chrono::Local::now();
    let status = time.format("%F %I:%M:%S %p").to_string();

    println!("{}", status);

    std::io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs_f64(1.0 / 60.0));
  }
}

