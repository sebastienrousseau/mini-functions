// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate image;
use image::{imageops, ImageBuffer, Rgba, RgbaImage};
extern crate qrc;
use self::qrc::QRCode;

use std::fs;

const URL: &str = "https://minifunctions.com/";

fn main() {
    generate_and_process_qrcode();
}

fn generate_and_process_qrcode() {
    let qrcode = QRCode::from_string(URL.to_string());
    process_png(&qrcode, "qrcode.png");
    process_png_with_custom_color(&qrcode, "qrcode_colorized.png");
    process_svg(&qrcode, "qrcode.svg");
    process_resized_qrcode(&qrcode, "qrcode_resized.png");
    // Add additional QR code processing functions here as needed.
}

fn process_png(qrcode: &QRCode, filename: &str) {
    let png = qrcode.to_png(512);
    save_and_remove_file(png.into_raw(), filename, 512, 512);
}

fn process_png_with_custom_color(qrcode: &QRCode, filename: &str) {
    let red = Rgba([255, 0, 0, 255]);
    let red_qrcode = qrcode.colorize(red);
    let img: RgbaImage = red_qrcode;
    let resized_img = imageops::resize(&img, 512, 512, imageops::FilterType::Nearest);
    save_and_remove_file(resized_img.into_raw(), filename, 512, 512);
}

fn process_svg(qrcode: &QRCode, filename: &str) {
    let svg_data = qrcode.to_svg(512);
    match fs::write(filename, svg_data) {
        Ok(_) => println!("🦀 SVG file created: ✅ {}", filename),
        Err(e) => println!("🦀 SVG file creation failed: ❌ {}: {}", filename, e),
    }
    remove_file(filename);
}

fn process_resized_qrcode(qrcode: &QRCode, filename: &str) {
    let png = qrcode.to_png(512);
    let img: RgbaImage = ImageBuffer::from_raw(512, 512, png.into_raw()).unwrap();
    let resized_img = imageops::resize(&img, 256, 256, imageops::FilterType::Nearest); // Example resizing
    save_and_remove_file(resized_img.into_raw(), filename, 256, 256);
}

fn save_and_remove_file(data: Vec<u8>, filename: &str, width: u32, height: u32) {
    let image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, data).unwrap();
    match image.save(filename) {
        Ok(_) => println!("🦀 PNG file created: ✅ {}", filename),
        Err(e) => println!("🦀 PNG file creation failed: ❌ {}: {}", filename, e),
    }
    remove_file(filename);
}

fn remove_file(filename: &str) {
    match fs::remove_file(filename) {
        Ok(_) => println!("🦀 File removed: ✅ {}", filename),
        Err(e) => println!("🦀 File removal failed: ❌ {}: {}", filename, e),
    }
}
