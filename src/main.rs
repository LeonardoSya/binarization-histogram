use image::imageops::grayscale;
use image::{GrayImage, Luma};
use plotters::prelude::*;
use std::path::Path;

fn process_image(image_path: &str, threshold: u8) -> GrayImage {
    let img = image::open(&Path::new(image_path)).expect("无法读取图像文件");
    let mut gray_image = grayscale(&img);
    for pixel in gray_image.enumerate_pixels_mut() {
        let Luma([gray_value]) = *pixel.2;
        *pixel.2 = match gray_value {
            0..=63 => Luma([0]),
            64..=127 => Luma([85]),
            128..=191 => Luma([170]),
            _ => Luma([255]),
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

fn draw_histogram(histogram: &[u32; 256], output_file: &str) {
    let root_area = BitMapBackend::new(output_file, (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let max_value = *histogram.iter().max().unwrap() as i32;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("GrayScale Histogram", ("sans-serif", 50))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..255, 0..max_value)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            histogram.iter().enumerate().map(|(x, &y)| {
                Rectangle::new([(x as i32, 0), (x as i32, y as i32)], BLUE.filled())
            }),
        )
        .unwrap();
}
fn main() {
    let image_path = "src/img.png";
    let threshold = 128;
    let binary_image = process_image(image_path, threshold);
    binary_image
        .save("src/binary_image.png")
        .expect("无法保存二值化图像");
    let histogram = generate_histogram(&binary_image);
    draw_histogram(&histogram, "src/histogram.png");
}
