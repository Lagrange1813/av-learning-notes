use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

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
