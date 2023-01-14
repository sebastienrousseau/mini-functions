extern crate image;
use image::{ImageBuffer, Rgb, RgbImage}; // Import the ImageBuffer and Rgb structs from the image crate
extern crate mini_functions_qrcode;
use self::mini_functions_qrcode::QRCode;
use std::convert::TryInto;
use std::fs; // Import the fs module from the standard library // Import the QRCode struct from the mini_functions crate

fn main() {
    // Create a new QRCode using the QRCode::new() function
    let data = vec![0x61, 0x62, 0x63]; // Define a variable for the data to be encoded
    let qrcode = QRCode::new(data); // Create a new QRCode using the QRCode::new() function
    println!("🦀 fn new():              ✅ {}", qrcode.to_svg()); // Print the SVG representation of the QRCode

    // Create a new QRCode using the QRCode::from_string() function
    let data = "abc".to_string(); // Define a variable for the data to be encoded
    let qrcode = QRCode::from_string(data); // Create a new QRCode using the QRCode::from_string() function
    println!("🦀 fn from_string():      ✅ {}", qrcode.to_svg()); // Print the SVG representation of the QRCode

    // Create a new QRCode using the QRCode::from_bytes() function
    let data = vec![0x61, 0x62, 0x63]; // Define a variable for the data to be encoded
    let qrcode = QRCode::from_bytes(data); // Create a new QRCode using the QRCode::from_bytes() function
    println!("🦀 fn from_bytes():       ✅ {}", qrcode.to_svg()); // Print the SVG representation of the QRCode

    // Create a new QRCode using the QRCode::from_string() function and convert it to a PNG representation
    let qrcode = QRCode::from_string("Hello, world!".to_string()); // Create a new QRCode using the QRCode::from_string() function
    let png = qrcode.to_png(); // Convert the QRCode into a PNG representation
    let png_data = png.into_raw(); // Convert the PNG representation of the QRCode into a vector of bytes
    let png_image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(21, 21, png_data).unwrap();
    println!(
        "🦀 fn to_png():           ✅ {:?}",
        png_image.save("qrcode.png")
    ); // Print the PNG representation of the QRCode
    png_image.save("qrcode.png").unwrap(); // Save the PNG representation of the QRCode to a file called "qrcode1.png"
    println!("🦀 file created:          ✅ qrcode.png"); // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode1.png"
    fs::remove_file("qrcode.png").unwrap(); // Remove the file that was created after the test.
    println!("🦀 file removed:          ✅ qrcode.png"); // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode1.png"

    // Create a new QRCode using the QRCode::from_string() function and convert it to a PNG representation with a custom color
    let qrcode = QRCode::new(vec![0, 1, 2, 3]); // Create a new QR code with some data.
    let red_qrcode = qrcode.colorize(Rgb([255, 0, 0])); // Colorize the QR code with a red color.
    let image: ImageBuffer<Rgb<u8>, Vec<u8>> = red_qrcode.unwrap(); // Convert the colorized QR code to a PNG image.
    println!(
        "🦀 fn colorize():         ✅ {:?}",
        image.save("qrcode_colorized.png")
    ); // Print the PNG representation of the QRCode
    image.save("qrcode_colorized.png").unwrap(); // Save the image to a file.
    println!("🦀 file created:          ✅ qrcode_colorized.png"); // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
    fs::remove_file("qrcode_colorized.png").unwrap(); // Remove the file that was created after the test.
    println!("🦀 file removed:          ✅ qrcode_colorized.png"); // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"

    // Create a new QRCode using the QRCode::from_string() function and convert it to a PNG representation with a custom size
    let qrcode = QRCode::new(vec![0x61, 0x62, 0x63]);
    let resized_image: RgbImage = qrcode.resize(512, 512).unwrap();
    println!(
        "🦀 fn resize():           ✅ {:?}",
        resized_image.save("qrcode_resized.png")
    ); // Print the PNG representation of the QRCode
    resized_image.save("qrcode_resized.png").unwrap(); // Save the image to a file.
    println!("🦀 file created:          ✅ qrcode_resized.png"); // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
    fs::remove_file("qrcode_resized.png").unwrap(); // Remove the file that was created after the test.
    println!("🦀 file removed:          ✅ qrcode_resized.png"); // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
}
