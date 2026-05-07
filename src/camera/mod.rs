use nokhwa::{Camera as NokhwaCamera, pixel_format::RgbFormat, utils::{CameraIndex, CameraFormat, FrameFormat, RequestedFormat, RequestedFormatType, Resolution}};
use image::{Rgb};

pub struct Camera {
    camera: NokhwaCamera,
}

impl Camera {
    pub fn new() -> Camera {
        let index = CameraIndex::Index(0);
        let cam_format = CameraFormat::new(
            Resolution::new(1280, 720),
            FrameFormat::RAWRGB,
            30
        );
        let requested = RequestedFormat::new::<RgbFormat>(
            RequestedFormatType::Exact(cam_format)
        );
        let mut camera = NokhwaCamera::new(index, requested).unwrap();
        camera.open_stream().unwrap();
          
        Camera { camera }
    }


    // l, or luminance in color science, apparently. eh, who knew?
    fn rgb_vec_to_l_matrix(&self, pixels: image::buffer::Pixels<'_, Rgb<u8>>) -> Vec<Vec<u8>> {
        // go from [R, G, B, R, G ,B ...] to [l, l, l ...]
        let l_vec = pixels.map(|rgb| {
                let (r, g, b) = (rgb[0] as i32, rgb[1] as i32, rgb[2] as i32);

                let result = (r + g + b) / 3;

                result as u8
            })
            .collect::<Vec<u8>>();

        let cam_w = self.camera.camera_format().resolution().width() as usize;

        l_vec.chunks(cam_w)
            .map(|x| x.to_vec())
            .collect()
    }

    pub fn capture(&mut self) -> Vec<Vec<u8>> {
        let (term_w, term_h) = asciicam::term_size().unwrap_or(crate::constants::DEFAULT_TERM_SIZE);
        let frame = self.camera.frame().unwrap();
        let buf = frame.decode_image::<RgbFormat>().unwrap();
        let l_matrix = self.rgb_vec_to_l_matrix(buf.pixels());

        // we gotta sample the frame to fit in tha terminal window
        let cam_h = l_matrix.len();
        let cam_w = l_matrix.first().map(|r| r.len()).unwrap_or(0);

        // no need to average the up/down sampling, just yoink the nearest neighbour
        (0..term_h).map(|ty| {
            let sy = ty * cam_h / term_h;
            (0..term_w).map(|tx| {
                let sx = tx * cam_w / term_w;

                // return and bubble up
                l_matrix[sy][sx]
            }).collect()
        }).collect()
    }
}
