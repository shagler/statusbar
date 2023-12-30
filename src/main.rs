use chrono;

#[derive(Debug)]
pub enum Error {
    OpenDisplayError,
    RootWindowError,
    FlushError,
    StoreNameError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::OpenDisplayError => write!(f, "Failed to open X display"),
            Error::RootWindowError => write!(f, "Failed to get root window"),
            Error::FlushError => write!(f, "Failed to flush X display"),
            Error::StoreNameError => write!(f, "Failed to store window name"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

mod x11 {
    use super::*;

    pub struct Display {
        pub display: usize,
    }

    impl Display {
        pub fn open() -> Result<Self> {
            let display = unsafe { XOpenDisplay(0) };
            if display == 0 {
                Err(Error::OpenDisplayError)
            }
            else {
                Ok(Self { display: display })
            }
        }

        pub fn default_root_window(&self) -> Result<usize> {
            let window = unsafe { XDefaultRootWindow(self.display) };
            if window == 0 {
                Err(Error::RootWindowError)
            }
            else {
                Ok(window)
            }
        }

        pub fn flush(&self) -> Result<()> {
            let result = unsafe { XFlush(self.display) };
            if result == 1 {
                Ok(())
            }
            else {
                Err(Error::FlushError)
            }
        }

        pub fn store_name(&self, window: usize, name: *const u8) -> Result<()> {
            let result = unsafe { 
                XStoreName(self.display, window, name)
            };
            if result == 1 {
                Ok(())
            }
            else {
                Err(Error::StoreNameError)
            }
        }
    }

    impl Drop for Display {
        fn drop(&mut self) {
            unsafe { XCloseDisplay(self.display); }
        }
    }
}

#[link(name = "X11")]
extern { 
    fn XOpenDisplay(name: usize) -> usize;
    fn XCloseDisplay(display: usize);
    fn XDefaultRootWindow(display: usize) -> usize;
    fn XFlush(display: usize) -> i32;
    fn XStoreName(display: usize, window: usize, name: *const u8) -> i32;
}

pub struct StatusBar {
    display: x11::Display,
    window: usize,
}

impl StatusBar {
    pub fn new() -> Result<Self> {
        let display = x11::Display::open()?;
        let window = display.default_root_window()?;

        Ok(StatusBar { display, window })
    }

    pub fn update_status(&self, status: &str) -> Result<()> {
        self.display.store_name(self.window, status.as_ptr())?;
        self.display.flush()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let status_bar = StatusBar::new()?;

    loop {
        let time = chrono::Local::now();
        let status = format!("{}\0", time.format("%F %T"));
        status_bar.update_status(&status)?;

        std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / 60.0));
    }
}
