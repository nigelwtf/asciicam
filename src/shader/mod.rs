use std::time;

const CHAR_MAP: [char; 20] = [
    ' ', '.', ':', '-', '=', '+', '*', '#', '%', '@', '░', '░', '▒', '▒', '▓', '▓', '█', '█', '█', '█'];

pub struct Shader {
    start_time: time::Instant,
}

impl Shader {
    pub fn new(start_time: time::Instant) -> Self {
        Shader { start_time }
    }

    fn empty_frame(&self) -> Vec<String> {
        let (w, h) = asciicam::term_size().unwrap_or(crate::constants::DEFAULT_TERM_SIZE);
        vec![" ".repeat(w); h]
    }

    fn status_bar(&self) -> String {
        let elapsed = self.start_time.elapsed().as_secs();
        format!("Ctrl + C to exit... Running for {elapsed}s")
    }

    pub fn render(&self, capture: Vec<Vec<u8>>) {
        let mut frame = self.empty_frame();
        let frame_len = frame.len();

        for (y, row) in capture.iter().enumerate() {
            if y >= frame_len - 1 { break; }
            frame[y] = row.iter().map(|&l| {
                let i = (l as usize * (CHAR_MAP.len() - 1)) / 255;
                CHAR_MAP[i]
            }).collect();
        }

        frame[frame_len - 1] = self.status_bar();

        for (i, line) in frame.iter().enumerate() {
            print!("\x1b[{};1H{}", i + 1, line);
        }
    }
}