use image::imageops::grayscale;
use image::{GenericImageView, GrayImage, Luma};
use std::path::Path;

fn process_image(image_path: &str, threshold: u8) -> GrayImage {
    let img = image::open(&Path::new(image_path)).expect("无法读取图像文件");
    let mut gray_image = grayscale(&img);
    for pixel in gray_image.enumerate_pixels_mut() {
        let Luma([gray_value]) = *pixel.2;
        *pixel.2 = if gray_value >= threshold {
            Luma([255])
        } else {
            Luma([0])
        };
    }
    gray_image
}

fn generate_histogram(image: &GrayImage) -> [u32; 256] {
    let mut histogram = [0u32; 256];
    for pixel in image.pixels() {
        let gray_value = pixel[0] as usize;
        histogram[gray_value] += 1;
    }
    histogram
}

fn main() {
    let image_path = "src/img.png";
    let threshold = 128;
    let binary_image = process_image(image_path, threshold);
    binary_image
        .save("src/binary_image.png")
        .expect("无法保存二值化图像");
    let histogram = generate_histogram(&binary_image);
    for (i, &count) in histogram.iter().enumerate() {
        print!("灰度值 {}: {}", i, count);
    }
}
