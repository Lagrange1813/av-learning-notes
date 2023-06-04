use super::super::picture::rgb;
use super::super::picture::yuv;

const INPUT_DIR: &str = "input/";
const OUTPUT_DIR: &str = "output/";

/// Run examples of picture module
pub fn run() {
    match yuv::yuv420_split(
        &format!("{}lena_256x256_yuv420p.yuv", INPUT_DIR),
        256,
        256,
        1,
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: yuv420_split {:?}", err),
    };

    match yuv::yuv444_split(
        &format!("{}lena_256x256_yuv444p.yuv", INPUT_DIR),
        256,
        256,
        1,
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: yuv444_split {:?}", err),
    };

    match yuv::yuv420_grey(
        &format!("{}lena_256x256_yuv420p.yuv", INPUT_DIR),
        256,
        256,
        1,
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: yuv420_grey {:?}", err),
    }

    match yuv::yuv420_halfy(
        &format!("{}lena_256x256_yuv420p.yuv", INPUT_DIR),
        256,
        256,
        1,
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: yuv420_halfy {:?}", err),
    }

    match yuv::yuv420_border(
        &format!("{}lena_256x256_yuv420p.yuv", INPUT_DIR),
        256,
        256,
        20,
        1,
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: yuv420_border {:?}", err),
    }

    match yuv::yuv420_greybar(
        640,
        360,
        0,
        255,
        10,
        &format!("{}graybar_640x360.yuv", OUTPUT_DIR),
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: yuv420_greybar {:?}", err),
    }

    match yuv::yuv420_psnr(
        &format!("{}lena_256x256_yuv420p.yuv", INPUT_DIR),
        &format!("{}lena_distort_256x256_yuv420p.yuv", INPUT_DIR),
        256,
        256,
        1,
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: yuv420_psnr {:?}", err),
    }

    match rgb::rgb24_split(
        &format!("{}cie1931_500x500.rgb", INPUT_DIR),
        500,
        500,
        1,
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: rgb24_split {:?}", err),
    }

    match rgb::rgb24_to_bmp(
        &format!("{}lena_256x256_rgb24.rgb", INPUT_DIR),
        256,
        256,
        &format!("{}output_lena.bmp", OUTPUT_DIR),
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: rgb24_to_bmp {:?}", err),
    }

    match rgb::rgb24_to_yuv420(
        &format!("{}lena_256x256_rgb24.rgb", INPUT_DIR),
        256,
        256,
        1,
        &format!("{}output_lena.yuv", OUTPUT_DIR),
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: rgb24_to_yuv420 {:?}", err),
    }

    match rgb::rgb24_colorbar(
        640,
        360,
        &format!("{}colorbar_640x360.rgb", OUTPUT_DIR),
    ) {
        Ok(_) => (),
        Err(err) => println!("Error: rgb24_colorbar {:?}", err),
    }
}
