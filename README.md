# sway statusbar

A custom statusbar output for the Sway window manager, written in Rust. This statusbar displays system information
including disk usage, memory usage, CPU usage, GPU usage, network status, and current time. 

## Installation

1. Clone this repository:
```
git clone https://github.com/shagler/statusbar.git
cd statusbar
```

2. Build and install the status bar:
```
cargo install --path .
```

## Configuration

1. Open your sway config file (usually located at `~/.config/sway/config`).

2. Remove any existing `bar {...}` sections.

3. Add the following line to your Sway config:
```
exec_always ~/.cargo/bin/statusbar
```

4. Restart Sway or reload the configuration.
