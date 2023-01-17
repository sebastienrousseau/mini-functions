extern crate image;
use image::{imageops, ImageBuffer, Rgb, RgbImage};
extern crate mini_functions_qrcode;
use self::mini_functions_qrcode::QRCode;
use std::fs; // Import the fs module from the standard library // Import the QRCode struct from the mini_functions crate

const URL: &str = "https://minifunctions.com/"; // Define a constant for the URL to be encoded

fn main() {
    // Create a new QRCode using the QRCode::from_string() function and convert it to a PNG representation
    let qrcode = QRCode::from_string(URL.to_string()); // Create a new QRCode using the QRCode::from_string() function
    let png = qrcode.to_png(); // Convert the QRCode into a PNG representation
    let png_data = png.into_raw(); // Convert the PNG representation of the QRCode into a vector of bytes
    let png_image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(21, 21, png_data).unwrap();
    println!(
        "🦀 fn to_png():                ✅ {:?}",
        png_image.save("qrcode.png")
    ); // Print the PNG representation of the QRCode
    match png_image.save("qrcode.png") {
        Ok(_) => println!("🦀 png file created:           ✅ qrcode.png"), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode1.png"
        Err(e) => println!("🦀 png file created:           ❌ qrcode.png: {}", e), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode1.png"
    }
    match fs::remove_file("qrcode.png") {
        Ok(_) => println!("🦀 png file removed:           ✅ qrcode.png"), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode1.png"
        Err(e) => println!("🦀 png file removed:           ❌ qrcode.png: {}", e), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode1.png"
    }

    // Create a new QRCode using the QRCode::from_string() function and convert it to a PNG representation with a custom color
    let qrcode = QRCode::from_string(URL.to_string());
    let red_qrcode = qrcode.colorize(Rgb([255, 0, 0])); // Colorize the QR code with a red color.
    let img: RgbImage = red_qrcode; // Convert the colorized QR code to a PNG image.
    let new_width = 512;
    let new_height = 512;
    let resized_img = imageops::resize(&img, new_width, new_height, imageops::FilterType::Nearest);
    let image: ImageBuffer<Rgb<u8>, Vec<u8>> = resized_img; // Convert the colorized QR code to a PNG image.
    println!(
        "🦀 fn colorize():              ✅ {:?}",
        image.save("qrcode_colorized.png")
    ); // Print the PNG representation of the QRCode
    match image.save("qrcode_colorized.png") {
        Ok(_) => println!("🦀 colorized png file created: ✅ qrcode_colorized.png"), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
        Err(e) => println!(
            "🦀 colorized png file created: ❌ qrcode_colorized.png: {}",
            e
        ), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
    }
    match fs::remove_file("qrcode_colorized.png") {
        Ok(_) => println!("🦀 colorized png file removed: ✅ qrcode_colorized.png"), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
        Err(e) => println!(
            "🦀 colorized png file removed: ❌ qrcode_colorized.png: {}",
            e
        ), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
    }

    // Create a new QRCode using the QRCode::from_string() function and convert it to an SVG representation
    let qrcode = QRCode::from_string(URL.to_string());
    let qrcode_svg = qrcode.to_svg(512); // Convert the QRCode into an SVG representation
    match fs::write("qrcode.svg", qrcode_svg) {
        Ok(_) => println!("🦀 svg file created:           ✅ qrcode.svg"), // Print the path to the SVG representation of the QRCode that was saved to a file called "qrcode.svg"
        Err(e) => println!("🦀 svg file created:           ❌ qrcode.svg: {}", e), // Print the path to the SVG representation of the QRCode that was saved to a file called "qrcode.svg"
    }
    match fs::remove_file("qrcode.svg") {
        Ok(_) => println!("🦀 svg file removed:           ✅ qrcode.svg"), // Print the path to the SVG representation of the QRCode that was saved to a file called "qrcode.svg"
        Err(e) => println!("🦀 svg file removed:           ❌ qrcode.svg: {}", e), // Print the path to the SVG representation of the QRCode that was saved to a file called "qrcode.svg"
    }

    // Create a new QRCode using the QRCode::from_string() function and convert it to a PNG representation with a custom size
    let qrcode = QRCode::new(vec![0x61, 0x62, 0x63]);
    let resized_image: RgbImage = qrcode.resize(512, 512);
    println!(
        "🦀 fn resize():                ✅ {:?}",
        resized_image.save("qrcode_resized.png")
    ); // Print the PNG representation of the QRCode
    match resized_image.save("qrcode_resized.png") {
        Ok(_) => println!("🦀 resized file created:       ✅ qrcode_resized.png"), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
        Err(e) => println!(
            "🦀 resized file created:       ❌ qrcode_resized.png: {}",
            e
        ), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
    }
    match fs::remove_file("qrcode_resized.png") {
        Ok(_) => println!("🦀 resized file removed:       ✅ qrcode_resized.png"), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
        Err(e) => println!(
            "🦀 resized file removed:       ❌ qrcode_resized.png: {}",
            e
        ), // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
    }
}
