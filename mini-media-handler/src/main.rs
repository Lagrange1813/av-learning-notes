use mini_media_handler::Handler;

fn main() {
    let handler = Handler::new();

    match handler.yuv420_split("tests/lena_256x256_yuv420p.yuv", 256, 256, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    };

    match handler.yuv444_split("tests/lena_256x256_yuv444p.yuv", 256, 256, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    };

    match handler.yuv420_grey("tests/lena_256x256_yuv420p.yuv", 256, 256, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match handler.yuv420_halfy("tests/lena_256x256_yuv420p.yuv", 256, 256, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match handler.yuv420_border("tests/lena_256x256_yuv420p.yuv", 256, 256, 20, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match handler.yuv420_greybar(640, 360,0,255,10,"output/graybar_640x360.yuv") {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }
}
