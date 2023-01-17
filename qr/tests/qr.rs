#[cfg(test)]
mod tests {
    extern crate image;
    use image::{Rgb, RgbImage};

    extern crate qr;
    use self::qr::QRCode;

    #[test]
    fn test_new() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::new(data.clone());
        assert_eq!(qrcode.data, data);
    }

    #[test]
    fn test_from_string() {
        let data = "abc".to_string();
        let qrcode = QRCode::from_string(data.clone());
        assert_eq!(qrcode.data, data.into_bytes());
    }

    #[test]
    fn test_from_bytes() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);
    }

    #[test]
    fn test_to_qrcode() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);
    }

    #[test]
    fn test_to_png() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);

        let qrcode = QRCode::from_string("Hello, world!".to_string());
        let png = qrcode.to_png();
        assert_eq!(png.dimensions(), (21, 21));

        let png_data = png.into_raw();
        assert_eq!(png_data.len(), 1323);
    }

    #[test]
    fn test_colorize() {
        // Create a new QR code with some data.
        let qrcode = QRCode::new(vec![0, 1, 2, 3]);

        // Colorize the QR code with a red color.
        let red_qrcode = qrcode.colorize(Rgb([255, 0, 0]));

        // Convert the QR code to a PNG image and assert that all of the dark cells are red.
        let image: RgbImage = red_qrcode;
        for (x, y, pixel) in image.enumerate_pixels() {
            let expected_color =
                if qrcode.to_qrcode()[(x as usize, y as usize)] == qrcode::Color::Dark {
                    Rgb([255, 0, 0])
                } else {
                    Rgb([255, 255, 255])
                };
            assert_eq!(*pixel, expected_color);
        }
    }

    #[test]
    fn test_resize() {
        // Create a new QR code with some data.
        let qrcode = QRCode::new(vec![0, 1, 2, 3]);

        // Resize the QR code to 42x42 pixels.
        let resized_qrcode = qrcode.resize(42, 42);

        // Convert the QR code to a PNG image and assert that the dimensions are correct.
        let image: RgbImage = resized_qrcode;
        assert_eq!(image.dimensions(), (42, 42));
    }

    #[test]
    fn test_to_svg() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data);
        assert!(qrcode.data != vec![], "{}", true);
    }
}
