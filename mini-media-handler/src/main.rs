mod picture;

use picture::yuv;
use picture::rgb;

fn main() {
    match yuv::yuv420_split("tests/lena_256x256_yuv420p.yuv", 256, 256, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    };

    match yuv::yuv444_split("tests/lena_256x256_yuv444p.yuv", 256, 256, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    };

    match yuv::yuv420_grey("tests/lena_256x256_yuv420p.yuv", 256, 256, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match yuv::yuv420_halfy("tests/lena_256x256_yuv420p.yuv", 256, 256, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match yuv::yuv420_border("tests/lena_256x256_yuv420p.yuv", 256, 256, 20, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match yuv::yuv420_greybar(640, 360, 0, 255, 10, "output/graybar_640x360.yuv") {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match yuv::yuv420_psnr("tests/lena_256x256_yuv420p.yuv", "tests/lena_distort_256x256_yuv420p.yuv", 256, 256, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match rgb::rgb24_split("tests/cie1931_500x500.rgb", 500, 500, 1) {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match rgb::rgb24_to_bmp("tests/lena_256x256_rgb24.rgb", 256, 256, "output/output_lena.bmp") {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match rgb::rgb24_to_yuv420("tests/lena_256x256_rgb24.rgb", 256, 256, 1, "output/output_lena.yuv") {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    match rgb::rgb24_colorbar(640, 360, "output/colorbar_640x360.rgb") {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }
}
