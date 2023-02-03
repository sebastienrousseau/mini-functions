//! # Core QR code functionality
//!
//! QRCode provides an easy way to generate a QR code image in PNG or
//! SVG format.
//!
#![warn(missing_docs)]
#![forbid(unsafe_code)]
// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

extern crate image;
extern crate qrcode;

use image::{ImageBuffer, Rgb, RgbImage};
use qrcode::render::svg;
use qrcode::QrCode;

#[non_exhaustive]
#[derive(Debug, Clone, Default, PartialEq)]
/// The `QRCode` struct for generating an image in PNG or SVG format.
///
/// This struct represents a QR code image from a given data.
///
/// - It can be used to generate a QR code image in PNG or SVG format.
/// - The data is a string or a vector of bytes.
/// - The QR code image can be in black and white or in color.
/// - It can be resized, colorized and converted to an image.
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
    pub fn to_png(&self, width: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, width);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let c = if qrcode[(x as usize, y as usize)] == qrcode::Color::Dark {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 255, 255])
            };
            *pixel = c;
            *pixel = c;
        }
        img
    }

    // pub fn to_png(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    //     let qrcode = self.to_qrcode();
    //     let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> =
    //         ImageBuffer::new(qrcode.width() as u32, qrcode.width() as u32);
    //     for (x, y, pixel) in img.enumerate_pixels_mut() {
    //         let c = if qrcode[(x as usize, y as usize)] == qrcode::Color::Dark {
    //             Rgb([0, 0, 0])
    //         } else {
    //             Rgb([255, 255, 255])
    //         };
    //         *pixel = c;
    //         *pixel = c;
    //     }
    //     img
    // }

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
    pub fn colorize(&self, color: Rgb<u8>) -> RgbImage {
        let qrcode = self.to_qrcode();
        let mut img: RgbImage = ImageBuffer::new(qrcode.width() as u32, qrcode.width() as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let c = if qrcode[(x as usize, y as usize)] == qrcode::Color::Dark {
                color
            } else {
                Rgb([255, 255, 255])
            };
            *pixel = c;
        }
        img
    }

    /// The `resize` method creates a new PNG image of the QR code using
    /// the data stored in the QRCode and the given width and height
    /// values to resize the QR code.
    pub fn resize(&self, width: u32, height: u32) -> RgbImage {
        let qrcode = self.to_qrcode();
        let mut img: RgbImage = ImageBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let x_index = (x as f32 / width as f32) * qrcode.width() as f32;
                let y_index = (y as f32 / height as f32) * qrcode.width() as f32;
                let c = match qrcode[(x_index as usize, y_index as usize)] {
                    qrcode::Color::Dark => Rgb([0, 0, 0]),
                    qrcode::Color::Light => Rgb([255, 255, 255]),
                };
                img.put_pixel(x, y, c);
            }
        }
        img
    }
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
/// Define a macro named `qr_code_from`
macro_rules! qr_code_from {
    // This macro takes two expressions: `$data` and `$format`
    ($data:expr, $format:expr,  $width:expr) => {
        // Match the value of `$format`
        match $format {
            // If `$format` is equal to "png", generate a PNG format QR code using `QRCode::from_bytes`
            "png" => QRCode::from_bytes($data).to_png($width),
            // For any other value, panic with the message "Invalid format"
            _ => panic!("Invalid format"),
        }
    };
}
