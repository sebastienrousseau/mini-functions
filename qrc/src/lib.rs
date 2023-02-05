// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for generating and manipulating QR code images in various formats
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-qrc.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/qrc.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/qrc/)
//! [![Docs.rs](https://img.shields.io/badge/docs.rs-v0.0.1-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://docs.rs/qrc)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.1-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/qrc)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions/tree/main/qrc)
//! [![License](https://img.shields.io/crates/l/qrc.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! Introducing the QRCode Library - a versatile tool for generating and
//! manipulating QR code images in various formats.
//!
//! With this library, you can easily convert your data into a QR code,
//! whether it be in the form of a string or a vector of bytes.
//!
//! Choose from popular image formats like PNG, JPG, and SVG, and even
//! customize the size and color of your QR code. And with its
//! comprehensive set of features, including support for different modes
//! like Numeric and Alphanumeric, and ability to mix modes, you can
//! create QR codes that perfectly suit your needs.
//!
//! The QRCode Library is a powerful and flexible solution for all your
//! QR code generation needs.
//!
//! ## Features
//!
//! The QRCode Library features a `QRCode` struct that can be
//! constructed with a `Vec<u8>` of data or a `String` of data that will
//! be converted to a `Vec<u8>`.
//!
//! The QR code can be generated using the zto_qrcode` method, and
//! specific image formats can be generated using the `to_png`, `to_jpg`,
//! and `to_gif` methods.
//!
//! Each of these methods takes a `width` parameter and returns an
//! `ImageBuffer` containing the QR code image.
//!
//! The library uses the qrcode and image crates to generate the QR
//! code images.
//!
//! As of the current version, the library supports the following
//! features with the following status:
//!
//! | Feature | Description |
//! | ------- | ----------- |
//! | Library license | Apache-2.0 OR MIT |
//! | Library version | 0.0.1 |
//! | Mode Numeric | not specified |
//! | Mode Alphanumeric | not specified |
//! | Mode Byte | not specified |
//! | Mode Kanji | not specified |
//! | Mode ECI | not specified |
//! | Mode FNC1 | not specified |
//! | Mode Structured Append | not specified |
//! | Mode Hanzi | not specified |
//! | Mixing modes | not specified |
//! | QR Codes version 1 - 40 | not specified |
//! | Micro QR Codes version M1 - M4 | not specified |
//! | Find maximal error correction level | not specified |
//! | Optimize QR Codes | not specified |
//! | PNG output | supported |
//! | JPG output | supported |
//! | GIF output | supported |
//! | SVG output | supported |
//! | EPS output | not specified |
//! | PDF output | not specified |
//! | BMP output | not specified |
//! | TIFF output | not specified |
//! | WebP output | not specified |
//! | Black and white QR Codes | Yes |
//! | Colorized QR code | Yes |
//! | Animated QR Codes (GIF, APNG, WebP) | not specified |
//! | Changing size of modules (scaling factor) | not specified |
//! | Command line script | not specified |
//! | QR code resizing | supported |
//! | QR code watermarking | supported |
//! | QR code with logo | supported |
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
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-qrc.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-qrc.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "qrc"]
#![crate_type = "lib"]

extern crate image;
extern crate qrcode;

use image::{ImageBuffer, Rgba, RgbaImage};
use qrcode::render::svg;
use qrcode::QrCode;

#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// QRCode is a structure that contains data in the form of a vector of
/// bytes.
pub struct QRCode {
    /// The `data` field holds the data to be encoded in the QR code.
    pub data: Vec<u8>,
}
/// Implementation of QRCode structure.
impl QRCode {
    /// Creates a new QRCode structure with the given data.
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

    /// Creates a new QRCode structure from a vector of bytes.
    pub fn from_bytes(data: Vec<u8>) -> Self {
        QRCode { data }
    }

    /// Converts the QRCode structure to a QrCode structure.
    pub fn to_qrcode(&self) -> QrCode {
        QrCode::new(&self.data).unwrap()
    }

    /// Converts the QRCode structure to a PNG image.
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
    /// Converts the QRCode structure to a JPG image.
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
    /// Converts the QRCode structure to a GIF image.
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

    /// Converts the QRCode structure to an SVG image.
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
}

#[macro_export]
/// The `add_emoji_watermark` macro creates a new instance of the QRCode struct
/// with the given data.
macro_rules! add_image_watermark {
    ($img:expr, $watermark:expr) => {
        QRCode::add_image_watermark($img, $watermark)
    };
}

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
