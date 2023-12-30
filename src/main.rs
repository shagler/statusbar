use chrono;

#[link(name = "X11")]
extern { 
    fn XOpenDisplay(monitor: usize) -> usize;
    fn XDefaultRootWindow(display: usize) -> usize;
    fn XFlush(display: usize) -> i32;
    fn XStoreName(display: usize, window: usize, name: *const u8) -> i32;
}

fn main() {
    let display = unsafe { XOpenDisplay(0) };
    let window = unsafe { XDefaultRootWindow(display) };

    loop {
        let time = chrono::Local::now();
        let status = format!("{}\0", time.format("%F %T"));

        unsafe { XStoreName(display, window, status.as_ptr()) };
        unsafe { XFlush(display) };

        std::thread::sleep(std::time::Duration::from_nanos((1e9 / 60.) as u64));
    }
}
