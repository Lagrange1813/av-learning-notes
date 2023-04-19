use super::super::audio;

const INPUT_DIR: &str = "input/";
const OUTPUT_DIR: &str = "output/";

/// Run examples of audio module
pub fn run() {
    match audio::pcm16le_split(
        &format!("{}NocturneNo2inEflat_44.1k_s16le.pcm", INPUT_DIR),
        OUTPUT_DIR,
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: pcm16le_split {:?}", err),
    }
}
