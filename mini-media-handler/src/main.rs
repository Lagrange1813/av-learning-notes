mod picture;
mod audio;

mod example;

use example::picture_example;
use example::audio_example;

use std::thread;

fn main() {
    let picture_handle = thread::spawn(|| {
        picture_example::run();
    });

    let audio_handle = thread::spawn(|| {
        audio_example::run();
    });

    picture_handle.join().unwrap();
    audio_handle.join().unwrap();
}
