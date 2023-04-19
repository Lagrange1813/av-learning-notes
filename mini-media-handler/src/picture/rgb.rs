use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec;
use std::mem::size_of;
use serde::{Serialize, Deserialize};

/// Split R, G, B planes in RGB24 file.
/// * `url` - Path to RGB24 file
/// * `width` - Width of RGB24 file
/// * `height` - Height of RGB24 file
/// * `num` - Number of frames to process
pub fn rgb24_split(
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

        let mut output_r = File::create("output/output_r.y")?;
        let mut output_g = File::create("output/output_g.y")?;
        let mut output_b = File::create("output/output_b.y")?;

        for i in 0..width * height {
            output_r.write(&[buffer[i * 3]])?;
            output_g.write(&[buffer[i * 3 + 1]])?;
            output_b.write(&[buffer[i * 3 + 2]])?;
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct BmpHeader {
    image_size: i32,
    blank: i32,
    start_position: i32,
}

#[derive(Serialize, Deserialize)]
struct InfoHeader {
    length: i32,
    width: i32,
    height: i32,
    color_plane: u16,
    bit_color: u16,
    zip_format: i32,
    real_size: i32,
    x_pels: i32,
    y_pels: i32,
    color_use: i32,
    color_important: i32,
}

/// Convert RGB24 file to BMP file.
/// * `path` - Path to RGB24 file
/// * `width` - Width of RGB24 file
/// * `height` - Height of RGB24 file
/// * `url_out` - Path to output BMP file
pub fn rgb24_to_bmp(
    path: &str,
    width: usize,
    height: usize,
    url_out: &str,
) -> Result<(), Box<dyn Error>> {
    let bf_type = [b'B', b'M'];
    let header_size = size_of::<BmpHeader>() + size_of::<InfoHeader>() + size_of::<[u8; 2]>();

    let rgb_path = Path::new(path);
    let mut rgb_file = File::open(&rgb_path)?;
    let mut buffer = vec![0; width * height * 3];
    rgb_file.read(&mut buffer)?;

    let bmp_header = BmpHeader {
        image_size: (width * height * 3 + header_size) as i32,
        blank: 0,
        start_position: header_size as i32,
    };

    let info_header = InfoHeader {
        length: size_of::<InfoHeader>() as i32,
        width: width as i32,
        height: -(height as i32),
        color_plane: 1,
        bit_color: 24,
        zip_format: 0,
        real_size: (width * height * 3) as i32,
        x_pels: 0,
        y_pels: 0,
        color_use: 0,
        color_important: 0,
    };

    let bmp_path = Path::new(url_out);
    let mut bmp_file = File::create(&bmp_path)?;

    bmp_file.write(&bf_type)?;
    bmp_file.write(&bincode::serialize(&bmp_header)?)?;
    bmp_file.write(&bincode::serialize(&info_header)?)?;

    // BMP save R1|G1|B1,R2|G2|B2 as B1|G1|R1,B2|G2|R2
    // It saves pixel data in Little Endian
    // So we change 'R' and 'B'
    for i in 0..width * height {
        let tmp = buffer[i * 3];
        buffer[i * 3] = buffer[i * 3 + 2];
        buffer[i * 3 + 2] = tmp;
    }

    bmp_file.write(&buffer)?;

    println!("Finish generate {url_out}!");

    Ok(())
}

fn rgb_to_yuv_conventer(
    rgb_buffer: &mut Vec<u8>,
    width: usize,
    height: usize,
    yuv_buffer: &mut Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    let mut y_idx = 0;
    let mut u_idx = width * height;
    let mut v_idx = width * height * 5 / 4;

    for i in 0..height {
        for j in 0..width {
            let r = rgb_buffer[i * width * 3 + j * 3] as i32;
            let g = rgb_buffer[i * width * 3 + j * 3 + 1] as i32;
            let b = rgb_buffer[i * width * 3 + j * 3 + 2] as i32;

            let y = (((66 * r + 129 * g + 25 * b + 128) >> 8) + 16) as u8;
            let u = (((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128) as u8;
            let v = (((112 * r - 94 * g - 18 * b + 128) >> 8) + 128) as u8;

            yuv_buffer[y_idx] = y;
            y_idx += 1;

            if i % 2 == 0 && j % 2 == 0 {
                yuv_buffer[u_idx] = u;
                yuv_buffer[v_idx] = v;
                u_idx += 1;
                v_idx += 1;
            }
        }
    }

    Ok(())
}

/// Convert RGB24 file to YUV420 file.
/// * `rgb_url` - Path to RGB24 file
/// * `width` - Width of RGB24 file
/// * `height` - Height of RGB24 file
/// * `yuv_url` - Path to output YUV420 file
pub fn rgb24_to_yuv420(
    rgb_url: &str,
    width: usize,
    height: usize,
    num: usize,
    yuv_url: &str,
) -> Result<(), Box<dyn Error>> {
    let rgb_path = Path::new(rgb_url);
    let mut rgb_file = File::open(&rgb_path)?;
    let mut rgb_buffer = vec![0; width * height * 3];

    let yuv_path = Path::new(yuv_url);
    let mut yuv_file = File::create(&yuv_path)?;
    let mut yuv_buffer: Vec<u8> = vec![0; width * height * 3 / 2];

    for _ in 0..num {
        rgb_file.read(&mut rgb_buffer)?;
        rgb_to_yuv_conventer(&mut rgb_buffer, width, height, &mut yuv_buffer)?;
        yuv_file.write(&yuv_buffer)?;
    }

    Ok(())
}

pub fn rgb24_colorbar(width: usize, height:usize, url_out: &str) -> Result<(), Box<dyn Error>> {
    let color = [
        [255, 255, 255],
        [255, 255, 0],
        [0, 255, 255],
        [0, 255, 0],
        [255, 0, 255],
        [255, 0, 0],
        [0, 0, 255],
        [0, 0, 0],
    ];

    let bar_width = width / 8;
    let mut buffer = vec![0; width * height * 3];

    for i in 0..height {
        for j in 0..width {
            let bar_num = j / bar_width;
            buffer[i * width * 3 + j * 3] = color[bar_num][0];
            buffer[i * width * 3 + j * 3 + 1] = color[bar_num][1];
            buffer[i * width * 3 + j * 3 + 2] = color[bar_num][2];
        }
    }

    let path = Path::new(url_out);
    let mut file = File::create(&path)?;
    file.write(&buffer)?;

    Ok(())
}