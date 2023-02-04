// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for generating QR code images in PNG, JPG, GIF and SVG format
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-qr.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/mini-functions)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.8-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/mini-functions)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions)
//! [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! QRCode provides an easy way to generate QR code images in PNG, JPG, GIF and SVG format.
//!
//! ## Features
//!
//! - Generate QR code images in PNG, JPG, GIF and SVG format.
//! - The data is a string or a vector of bytes.
//! - The QR code image can be in black and white or in color.
//! - It can be resized, colorized and converted to an image.
//! - It can add an image as a watermark to the QR code image.
//!
//! ## Usage
//!
//! - [`serde`][]: Enable serialization/deserialization via serde
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-qr.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-qr.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "qr"]
#![crate_type = "lib"]

extern crate image;
extern crate qrcode;

use image::{ImageBuffer, Rgba, RgbaImage};
use qrcode::render::svg;
use qrcode::QrCode;

#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// The `QRCode` struct for generating an image in PNG, JPG, GIF and SVG format.
///
/// This struct represents a QR code image from a given data.
///
///
///
pub struct QRCode {
    /// The `data` field holds the data to be encoded in the QR code.
    pub data: Vec<u8>,
}

impl QRCode {
    /// The `new` method creates a new instance of the QRCode struct
    /// with the given data.
    pub fn new(data: Vec<u8>) -> Self {
        QRCode { data }
    }

    /// The `from_string` method creates a new instance of the QRCode
    /// struct by converting the given string data into a vector of
    /// bytes
    pub fn from_string(data: String) -> Self {
        QRCode {
            data: data.into_bytes(),
        }
    }

    /// The `from_bytes` method creates a new instance of the QRCode
    /// struct with the given vector of bytes
    pub fn from_bytes(data: Vec<u8>) -> Self {
        QRCode { data }
    }

    /// The `to_qrcode` method creates a new QrCode value using the data
    /// stored in the QRCode struct.
    pub fn to_qrcode(&self) -> QrCode {
        QrCode::new(&self.data).unwrap()
    }

    /// The `to_png` method creates a new PNG image of the QR code using
    /// the data stored in the QRCode
    pub fn to_png(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        let height = width;
        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let x_index = (x as f32 / width as f32) * qrcode.width() as f32;
            let y_index = (y as f32 / height as f32) * qrcode.width() as f32;
            *pixel = match qrcode[(x_index as usize, y_index as usize)] {
                qrcode::Color::Dark => Rgba([0, 0, 0, 0]),
                qrcode::Color::Light => Rgba([255, 255, 255, 255]),
            };
        }
        img
    }
    /// The `to_jpg` method creates a new JPEG image of the QR code
    /// using the data stored in the QRCode
    pub fn to_jpg(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        let height = width;
        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let x_index = (x as f32 / width as f32) * qrcode.width() as f32;
            let y_index = (y as f32 / height as f32) * qrcode.width() as f32;
            *pixel = match qrcode[(x_index as usize, y_index as usize)] {
                qrcode::Color::Dark => Rgba([0, 0, 0, 0]),
                qrcode::Color::Light => Rgba([255, 255, 255, 255]),
            };
        }
        img
    }
    /// The `to_gif` method creates a new GIF image of the QR code using
    /// the data stored in the QRCode
    pub fn to_gif(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        let height = width;
        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let x_index = (x as f32 / width as f32) * qrcode.width() as f32;
            let y_index = (y as f32 / height as f32) * qrcode.width() as f32;
            *pixel = match qrcode[(x_index as usize, y_index as usize)] {
                qrcode::Color::Dark => Rgba([0, 0, 0, 0]),
                qrcode::Color::Light => Rgba([255, 255, 255, 255]),
            };
        }
        img
    }

    /// The `to_svg` method creates a new SVG image of the QR code using
    /// the data stored in the QRCode
    pub fn to_svg(&self, width: u32) -> String {
        let qrcode = self.to_qrcode();
        let svg_string = qrcode
            .render::<svg::Color>()
            .min_dimensions(width, width)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#FFFFFF"))
            .build();
        svg_string
    }

    /// The `colorize` method creates a new PNG image of the QR code
    /// using the data stored in the QRCode and the given color value to
    /// colorize the QR code.
    pub fn colorize(&self, color: Rgba<u8>) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let mut img: RgbaImage = ImageBuffer::new(qrcode.width() as u32, qrcode.width() as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let c = if qrcode[(x as usize, y as usize)] == qrcode::Color::Dark {
                color
            } else {
                Rgba([255, 255, 255, 255])
            };
            *pixel = c;
        }
        img
    }

    /// The `resize` method creates a new PNG image of the QR code using
    /// the data stored in the QRCode and the given width and height
    /// values to resize the QR code.
    pub fn resize(&self, width: u32, height: u32) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let mut img: RgbaImage = ImageBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let x_index = (x as f32 / width as f32) * qrcode.width() as f32;
                let y_index = (y as f32 / height as f32) * qrcode.width() as f32;
                let c = match qrcode[(x_index as usize, y_index as usize)] {
                    qrcode::Color::Dark => Rgba([0, 0, 0, 0]),
                    qrcode::Color::Light => Rgba([255, 255, 255, 255]),
                };
                img.put_pixel(x, y, c);
            }
        }
        img
    }
    /// The `add_image_watermark` method adds a watermark to the given image.
    pub fn add_image_watermark(img: &mut RgbaImage, watermark: &RgbaImage) {
        let (width, height) = img.dimensions();
        let (watermark_width, watermark_height) = watermark.dimensions();

        // position the watermark in the bottom right corner
        let x = width - watermark_width;
        let y = height - watermark_height;

        // draw the watermark on the QR code image
        for (dx, dy, watermark_pixel) in watermark.enumerate_pixels() {
            let x = x + dx;
            let y = y + dy;
            let qr_pixel = img.get_pixel(x, y);

            let alpha = (watermark_pixel[3] as f32) / 255.0;
            let new_r = (1.0 - alpha) * (qr_pixel[0] as f32) + alpha * (watermark_pixel[0] as f32);
            let new_g = (1.0 - alpha) * (qr_pixel[1] as f32) + alpha * (watermark_pixel[1] as f32);
            let new_b = (1.0 - alpha) * (qr_pixel[2] as f32) + alpha * (watermark_pixel[2] as f32);
            let new_a = (qr_pixel[3] as f32) + alpha * (255.0 - qr_pixel[3] as f32);

            let new_pixel = [new_r as u8, new_g as u8, new_b as u8, new_a as u8];
            img.put_pixel(x, y, image::Rgba(new_pixel));
        }
    }
    // The `add_emoji_watermark` method adds an emoji watermark to the
    // given image.
    // pub fn add_emoji_watermark(img: &mut RgbaImage) {
    //     let (width, height) = img.dimensions();

    //     // position the watermark in the center of the QR code
    //     let x = width / 2 - 10;
    //     let y = height / 2 - 10;

    //     let emoji = "🦀︎";
    //     let emoji_image =
    //         image::load_from_memory_with_format(emoji.as_bytes(), image::ImageFormat::Png);
    //     let emoji_image = match emoji_image {
    //         Ok(img) => img,
    //         Err(e) => {
    //             println!("Error decoding emoji: {}", e);
    //             return;
    //         }
    //     };
    //     let emoji_image = emoji_image.resize(20, 20, image::imageops::FilterType::Nearest);
    //     let emoji_image = emoji_image.to_rgb8();
    //     for (dx, dy, pixel) in emoji_image.enumerate_pixels() {
    //         img.put_pixel(x + dx, y + dy, *pixel);
    //     }
    // }
}

#[macro_export]
/// The `add_emoji_watermark` macro creates a new instance of the QRCode struct
/// with the given data.
macro_rules! add_image_watermark {
    ($img:expr, $watermark:expr) => {
        QRCode::add_image_watermark($img, $watermark)
    };
}

// #[macro_export]
// The `add_emoji_watermark` macro creates a new instance of the QRCode struct
// with the given data.
// macro_rules! add_emoji_watermark {
//     ($img:expr) => {
//         QRCode::add_emoji_watermark($img)
//     };
// }

#[macro_export]
/// The `qr_code` macro creates a new instance of the QRCode struct
/// with the given data.
macro_rules! qr_code {
    ($data:expr) => {
        QRCode::new($data)
    };
}

#[macro_export]
/// Define a macro named `qr_code_to`
macro_rules! qr_code_to {
    // This macro takes two expressions: `$data` and `$format`
    ($data:expr, $format:expr, $width:expr) => {
        // Match the value of `$format`
        match $format {
            // If `$format` is equal to "png", generate a PNG format QR code using `QRCode::from_bytes`
            "png" => QRCode::from_bytes($data).to_png($width),
            // If `$format` is equal to "jpg", generate a JPG format QR code using `QRCode::from_bytes`
            "jpg" => QRCode::from_bytes($data).to_jpg($width),
            // If `$format` is equal to "gif", generate a "gif" format QR code using `QRCode::from_bytes`
            "gif" => QRCode::from_bytes($data).to_gif($width),
            // For any other value, panic with the message "Invalid format"
            _ => panic!("Invalid format"),
        }
    };
}
