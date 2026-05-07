mod constants;
mod shader;
mod camera;

use std::{thread, time, io::Write};

// restore terminal on sigint
extern "C" fn on_sigint(_: libc::c_int) {
    print!("\x1b[?25h");   // show cursor
    print!("\x1b[?1049l"); // leave alternate screen
    std::process::exit(0);
}

fn main() {
    let start_time = time::Instant::now();
    
    unsafe {
        libc::signal(libc::SIGINT, on_sigint as libc::sighandler_t);
    }
    
    print!("\x1b[?1049h"); // alternate screen
    print!("\x1b[?25l"); // hide term cursor
    std::io::stdout().flush().unwrap();
    
    let mut camera = camera::Camera::new();
    let shader = shader::Shader::new(start_time);

    loop {
        shader.render(camera.capture());
        std::io::stdout().flush().unwrap();
    }
}
