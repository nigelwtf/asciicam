use std::{sync::mpsc, thread};
use nokhwa::{Camera as NokhwaCamera, pixel_format::{RgbFormat, YuyvFormat}, utils::{CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType}};

pub struct Camera {
    rx: mpsc::Receiver<Vec<Vec<u8>>>,
}

impl Camera {
    pub fn list_formats() {
        let probes = [
            ("highest fps", RequestedFormatType::AbsoluteHighestFrameRate),
            ("highest res", RequestedFormatType::AbsoluteHighestResolution),
        ];
        for (label, req_type) in probes {
            let mut cam = NokhwaCamera::new(
                CameraIndex::Index(0),
                RequestedFormat::new::<RgbFormat>(req_type),
            ).unwrap();
            cam.open_stream().unwrap();
            let fmt = cam.camera_format();
            println!("{label}: {}x{}\t{:?}\t{}fps",
                fmt.resolution().width(),
                fmt.resolution().height(),
                fmt.format(),
                fmt.frame_rate()
            );
        }
    }

    pub fn new() -> Camera {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut cam = open_camera();
            loop {
                if tx.send(capture_frame(&mut cam)).is_err() {
                    break;
                }
            }
        });
        Camera { rx }
    }

    pub fn capture(&self) -> Option<Vec<Vec<u8>>> {
        let mut latest = None;
        while let Ok(frame) = self.rx.try_recv() {
            latest = Some(frame);
        }
        latest
    }
}

fn open_camera() -> NokhwaCamera {
    let mut cam = NokhwaCamera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate),
    ).unwrap();
    cam.open_stream().unwrap();
    let fmt = cam.camera_format();
    eprintln!("camera: {}x{} {:?} {}fps",
        fmt.resolution().width(), fmt.resolution().height(), fmt.format(), fmt.frame_rate());
    cam
}

fn capture_frame(cam: &mut NokhwaCamera) -> Vec<Vec<u8>> {
    let (term_w, term_h) = asciicam::term_size().unwrap_or(crate::constants::DEFAULT_TERM_SIZE);
    let frame = cam.frame().unwrap();
    let fmt = cam.camera_format();
    let cam_w = fmt.resolution().width() as usize;
    let cam_h = fmt.resolution().height() as usize;

    match fmt.format() {
        FrameFormat::YUYV => {
            let raw = frame.buffer().to_vec();
            sample(term_w, term_h, cam_w, cam_h, |sy, sx| raw[(sy * cam_w + sx) * 2])
        }
        FrameFormat::NV12 => {
            let raw = frame.buffer().to_vec();
            sample(term_w, term_h, cam_w, cam_h, |sy, sx| raw[sy * cam_w + sx])
        }
        _ => {
            let buf = frame.decode_image::<YuyvFormat>().unwrap();
            let raw = buf.as_raw();
            sample(term_w, term_h, cam_w, cam_h, |sy, sx| {
                let base = (sy * cam_w + sx) * 3;
                let (r, g, b) = (raw[base] as u16, raw[base + 1] as u16, raw[base + 2] as u16);
                ((r + g + b) / 3) as u8
            })
        }
    }
}

fn sample(term_w: usize, term_h: usize, cam_w: usize, cam_h: usize, luma: impl Fn(usize, usize) -> u8) -> Vec<Vec<u8>> {
    (0..term_h).map(|ty| {
        let sy = ty * cam_h / term_h;
        (0..term_w).map(|tx| {
            let sx = tx * cam_w / term_w;
            luma(sy, sx)
        }).collect()
    }).collect()
}
