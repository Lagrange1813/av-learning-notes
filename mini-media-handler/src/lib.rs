use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec;

pub struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }

    /// Split Y, U, V planes in YUV420P file.
    /// * `url` - Path to YUV420P file
    /// * `width` - Width of YUV420P file
    /// * `height` - Height of YUV420P file
    /// * `num` - Number of frames to process
    pub fn yuv420_split(
        &self,
        url: &str,
        width: usize,
        height: usize,
        num: usize,
    ) -> Result<(), Box<dyn Error>> {
        let path = Path::new(url);
        let size = width * height * 3 / 2;
        let mut buffer = Vec::with_capacity(size);

        for _ in 0..num {
            let file = File::open(&path)?;

            file.take(size as u64).read_to_end(&mut buffer)?;

            let mut output_y = File::create("output/output_420_y.y")?;
            output_y.write(&buffer[0..width * height])?;

            let mut output_u = File::create("output/output_420_u.y")?;
            output_u.write(&buffer[width * height..width * height * 5 / 4])?;

            let mut output_v = File::create("output/output_420_v.y")?;
            output_v.write(&buffer[width * height * 5 / 4..width * height * 3 / 2])?;
        }

        Ok(())
    }

    /// Split Y, U, V planes in YUV444P file.
    /// * `url` - Path to YUV444P file
    /// * `width` - Width of YUV444P file
    /// * `height` - Height of YUV444P file
    /// * `num` - Number of frames to process
    pub fn yuv444_split(
        &self,
        url: &str,
        width: usize,
        height: usize,
        num: usize,
    ) -> Result<(), Box<dyn Error>> {
        let path = Path::new(url);
        let size = width * height * 3;
        let mut buffer = Vec::with_capacity(size);

        for _ in 0..num {
            let file = File::open(&path)?;

            file.take(size as u64).read_to_end(&mut buffer)?;

            let mut output_y = File::create("output/output_444_y.y")?;
            output_y.write(&buffer[0..width * height])?;

            let mut output_u = File::create("output/output_444_u.y")?;
            output_u.write(&buffer[width * height..width * height * 2])?;

            let mut output_v = File::create("output/output_444_v.y")?;
            output_v.write(&buffer[width * height * 2..width * height * 3])?;
        }

        Ok(())
    }

    /// Convert YUV420P file to gray picture
    /// * `url` - Path to YUV420P file
    /// * `width` - Width of YUV420P file
    /// * `height` - Height of YUV420P file
    /// * `num` - Number of frames to process
    pub fn yuv420_grey(
        &self,
        url: &str,
        width: usize,
        height: usize,
        num: usize,
    ) -> Result<(), Box<dyn Error>> {
        let path = Path::new(url);
        let size = width * height * 3 / 2;
        let mut buffer = Vec::with_capacity(size);

        for _ in 0..num {
            let file = File::open(&path)?;

            file.take(size as u64).read_to_end(&mut buffer)?;
            for i in width * height..size {
                buffer[i] = 128;
            }

            let mut output_y = File::create("output/output_gray.yuv")?;
            output_y.write(&buffer[0..size])?;
        }

        Ok(())
    }

    /// Halve Y value of YUV420P file
    /// * `url` - Path to YUV420P file
    /// * `width` - Width of YUV420P file
    /// * `height` - Height of YUV420P file
    /// * `num` - Number of frames to process
    pub fn yuv420_halfy(
        &self,
        url: &str,
        width: usize,
        height: usize,
        num: usize,
    ) -> Result<(), Box<dyn Error>> {
        let path = Path::new(url);
        let size = width * height * 3 / 2;
        let mut buffer = Vec::with_capacity(size);

        for _ in 0..num {
            let file = File::open(&path)?;

            file.take(size as u64).read_to_end(&mut buffer)?;
            for i in 0..width * height {
                buffer[i] /= 2;
            }

            let mut output_y = File::create("output/output_half.yuv")?;
            output_y.write(&buffer[0..size])?;
        }

        Ok(())
    }

    /// Add border for YUV420P file
    /// * `url` - Path to YUV420P file
    /// * `width` - Width of YUV420P file
    /// * `height` - Height of YUV420P file
    /// * `border` - Border size
    /// * `num` - Number of frames to process
    pub fn yuv420_border(
        &self,
        url: &str,
        width: usize,
        height: usize,
        border: usize,
        num: usize,
    ) -> Result<(), Box<dyn Error>> {
        let path = Path::new(url);
        let size = width * height * 3 / 2;
        let mut buffer = Vec::with_capacity(size);

        for _ in 0..num {
            let file = File::open(&path)?;

            file.take(size as u64).read_to_end(&mut buffer)?;

            for i in 0..height {
                for j in 0..width {
                    if i < border || i > height - border || j < border || j > width - border {
                        buffer[i * width + j] = 255;
                    }
                }
            }

            let mut output_y = File::create("output/output_border.yuv")?;
            output_y.write(&buffer[0..size])?;
        }

        Ok(())
    }

    /// Generate YUV420P gray scale bar.
    /// * `width` - Width of YUV420P file
    /// * `height` - Height of YUV420P file
    /// * `ymin` - Minimum luminance
    /// * `ymax` - Maximum luminance
    /// * `barnum` - Number of bars
    /// * `output` - Path to output file
    pub fn yuv420_greybar(
        &self,
        width: usize,
        height: usize,
        ymin: u8,
        ymax: u8,
        barnum: usize,
        output: &str,
    ) -> Result<(), Box<dyn Error>> {
        let lum_inc: f64 = (ymax - ymin) as f64 / (barnum - 1) as f64;
        let barwidth = width / barnum;

        let mut y_buffer = vec![0; width * height];

        for i in 0..height {
            for j in 0..width {
                y_buffer[i * width + j] = ymin as u8 + ((j / barwidth) as f64 * lum_inc) as u8;
            }
        }

        let uv_width = width / 2;
        let uv_height = height / 2;

        let u_buffer = vec![128; uv_width * uv_height];
        let v_buffer = vec![128; uv_width * uv_height];

        let path = Path::new(output);
        let mut file = File::create(&path)?;

        file.write(&y_buffer)?;
        file.write(&u_buffer)?;
        file.write(&v_buffer)?;

        Ok(())
    }
}
