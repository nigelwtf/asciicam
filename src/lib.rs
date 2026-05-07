use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::io::{Error, Result};

pub fn term_size() -> Result<(usize, usize)> {
    let mut s = winsize {
        ws_col: 0,
        ws_row: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut s) };
    match r {
        0 => Ok((s.ws_col as usize, s.ws_row as usize)),
        _ => Err(Error::last_os_error()),
    }
}
