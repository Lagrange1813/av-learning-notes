use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::vec;

/// Split Y, U, V planes in YUV420P file.
/// * `url` - Path to YUV420P file
/// * `width` - Width of YUV420P file
/// * `height` - Height of YUV420P file
/// * `num` - Number of frames to process
pub fn yuv420_split(
    url: &str,
    width: usize,
    height: usize,
    num: usize,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(url);
    let mut file = File::open(&path)?;

    let size = width * height * 3 / 2;
    let mut buffer = vec![0; size];

    for _ in 0..num {
        file.read(&mut buffer)?;

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
    url: &str,
    width: usize,
    height: usize,
    num: usize,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(url);
    let mut file = File::open(&path)?;

    let size = width * height * 3;
    let mut buffer = vec![0; size];

    for _ in 0..num {
        file.read(&mut buffer)?;

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
    url: &str,
    width: usize,
    height: usize,
    num: usize,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(url);
    let mut file = File::open(&path)?;

    let size = width * height * 3 / 2;
    let mut buffer = vec![0; size];

    for _ in 0..num {
        file.read(&mut buffer)?;
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
    url: &str,
    width: usize,
    height: usize,
    num: usize,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(url);
    let mut file = File::open(&path)?;

    let size = width * height * 3 / 2;
    let mut buffer = vec![0; size];

    for _ in 0..num {
        file.read(&mut buffer)?;
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
    url: &str,
    width: usize,
    height: usize,
    border: usize,
    num: usize,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(url);
    let mut file = File::open(&path)?;

    let size = width * height * 3 / 2;
    let mut buffer = vec![0; size];

    for _ in 0..num {
        file.read(&mut buffer)?;

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
/// * `url_out` - Path to output file
pub fn yuv420_greybar(
    width: usize,
    height: usize,
    ymin: u8,
    ymax: u8,
    barnum: usize,
    url_out: &str,
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

    let path = Path::new(url_out);
    let mut file = File::create(&path)?;

    file.write(&y_buffer)?;
    file.write(&u_buffer)?;
    file.write(&v_buffer)?;

    Ok(())
}

// Calculate PSNR between 2 YUV420P file
// * `url1` - Path to YUV420P file 1
// * `url2` - Path to YUV420P file 2
// * `width` - Width of YUV420P file
// * `height` - Height of YUV420P file
// * `num` - Number of frames to process
pub fn yuv420_psnr(
    url1: &str,
    url2: &str,
    width: usize,
    height: usize,
    num: usize,
) -> Result<(), Box<dyn Error>> {
    let path1 = Path::new(url1);
    let path2 = Path::new(url2);

    let mut file1 = File::open(&path1)?;
    let mut file2 = File::open(&path2)?;

    let size = width * height * 3 / 2;
    let mut buffer1 = vec![0; size];
    let mut buffer2 = vec![0; size];

    for _ in 0..num {
        file1.read(&mut buffer1)?;
        file2.read(&mut buffer2)?;

        let mut mse_sum: f64 = 0.0;
        for i in 0..width * height {
            let diff = buffer1[i] as i32 - buffer2[i] as i32;
            mse_sum += diff as f64 * diff as f64;
        }

        let mse = mse_sum / (width * height) as f64;
        let psnr = 10.0 * f64::log10(255.0 * 255.0 / mse);

        println!("MSE: {:.3}, PSNR: {:5.3}", mse, psnr);

        // Skip UV
        file1.seek(SeekFrom::Current((width * height / 2) as i64))?;
    }

    Ok(())
}
