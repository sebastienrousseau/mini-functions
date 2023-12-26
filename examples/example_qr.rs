// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use image::{Rgba, RgbaImage};
use mini_functions::qr::QRCode;
use mini_functions::qr::{add_image_watermark, qr_code_to};
extern crate image;
use std::fs;

const URL: &str = "https://minifunctions.com/";

fn add_watermark_to_qrcode(
    qrcode: &mut RgbaImage,
    watermark_path: &str,
) -> Result<(), String> {
    match image::open(watermark_path) {
        Ok(watermark_img) => {
            let watermark_rgba = watermark_img.into_rgba8();
            add_image_watermark!(qrcode, &watermark_rgba);
            Ok(())
        }
        Err(e) => Err(format!("Failed to open watermark image: {}", e)),
    }
}

fn process_qrcode<F>(url: &str, process_fn: F, file_name: &str)
where
    F: FnOnce(&QRCode) -> RgbaImage,
{
    let qrcode = QRCode::from_string(url.to_string());
    let image = process_fn(&qrcode);
    save_image(&image, file_name);
}

fn save_image(image: &RgbaImage, file_name: &str) {
    match image.save(file_name) {
        Ok(_) => println!("🦀 File created: ✅ {}", file_name),
        Err(e) => {
            println!("🦀 File creation failed: ❌ {}: {}", file_name, e)
        }
    }
}

fn save_svg(data: &str, file_name: &str) {
    match fs::write(file_name, data) {
        Ok(_) => println!("🦀 File created: ✅ {}", file_name),
        Err(e) => {
            println!("🦀 File creation failed: ❌ {}: {}", file_name, e)
        }
    }
}

fn remove_file(file_name: &str) {
    match fs::remove_file(file_name) {
        Ok(_) => println!("🦀 File removed: ✅ {}", file_name),
        Err(e) => {
            println!("🦀 File removal failed: ❌ {}: {}", file_name, e)
        }
    }
}

fn main() {
    // Generate QR Code, save it as a PNG file and remove it after.
    process_qrcode(URL, |qrcode| qrcode.to_png(512), "qrcode.png");
    remove_file("qrcode.png");

    // Generate QR colorized QR Code, save it as a PNG file and remove it after.
    process_qrcode(
        URL,
        |qrcode| qrcode.colorize(Rgba([255, 0, 0, 255])),
        "qrcode_colorized.png",
    );
    remove_file("qrcode_colorized.png");

    // Generate QR Code, resize it and save it as a PNG file and remove it after.
    process_qrcode(
        URL,
        |qrcode| qrcode.resize(512, 512),
        "qrcode_resized.png",
    );
    remove_file("qrcode_resized.png");

    // SVG creation with decoupled functions
    let qrcode = QRCode::from_string(URL.to_string());
    let qrcode_svg = qrcode.to_svg(512);
    save_svg(&qrcode_svg, "qrcode.svg");
    remove_file("qrcode.svg");

    // QRCode with custom data
    let custom_qrcode = QRCode::new(vec![0x61, 0x62, 0x63]);
    let custom_qr_image = custom_qrcode.resize(512, 512);
    save_image(&custom_qr_image, "qrcode_custom.png");
    remove_file("qrcode_custom.png");

    // Create a new QRCode using the macro qr_code_to into a PNG representation with a custom size of 512x512
    // Note: Assuming you have these macros implemented
    let qrcode_png = qr_code_to!(URL.into(), "png", 512);
    save_image(&qrcode_png, "qrcode_macro.png");
    remove_file("qrcode_macro.png");

    // Create a new QRCode using the macro qr_code_to into a GIF representation with a custom size of 512x512
    let qrcode_gif = qr_code_to!(URL.into(), "gif", 512);
    save_image(&qrcode_gif, "qrcode_macro.gif");
    remove_file("qrcode_macro.gif");

    // Add watermark to QRCode and save
    let watermark_qrcode = QRCode::from_string(URL.to_string());
    let mut watermark_qr_img = watermark_qrcode.to_png(512);

    match add_watermark_to_qrcode(&mut watermark_qr_img, "bubba.ico") {
        Ok(_) => {
            save_image(&watermark_qr_img, "qrcode_watermarked.png")
        }
        Err(e) => println!("🦀 Error adding watermark: {}", e),
    }

    remove_file("qrcode_watermarked.png");
}
