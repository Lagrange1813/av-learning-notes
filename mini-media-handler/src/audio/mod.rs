use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Split Left and Right channel of 16LE PCM file.
/// * `input_url` - Path to PCM file
/// * `output_dir` - Path to output directory
pub fn pcm16le_split(input_url: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    let input_path = Path::new(input_url);
    let mut input_file = File::open(&input_path)?;
    let mut buffer = vec![0; 4];

    let mut output_file_l = File::create(format!("{}/output_l.pcm", output_dir))?;
    let mut output_file_r = File::create(format!("{}/output_r.pcm", output_dir))?;

    loop {
        match input_file.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                output_file_l.write(&buffer[0..2])?;
                output_file_r.write(&buffer[2..4])?;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(())
}

/// Half volume of Left channel of 16LE PCM file.
/// * `input_url` - Path to PCM file
/// * `output_dir` - Path to output directory
pub fn pcm16le_halfvolumeleft(input_url: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    let input_path = Path::new(input_url);
    let mut input_file = File::open(&input_path)?;
    let mut buffer = vec![0; 4];

    let mut output_file = File::create(format!("{}output_halfleft.pcm", output_dir))?;

    loop {
        match input_file.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let v = ((buffer[1] as i16) << 8) + buffer[0] as i16;
                let v = v / 2;
                let vl = v as u8;
                let vh = (v >> 8) as u8;
                output_file.write(&[vl, vh, buffer[2], buffer[3]])?;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(())
}
