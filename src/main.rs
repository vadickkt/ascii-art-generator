use image::{GenericImageView, ImageReader};

fn main() {
    let image = ImageReader::open("images/image_5x5.jpg")
        .expect("Image not found")
        .decode()
        .expect("Failed to decode image");

    let (width, height) = image.dimensions();
    println!("Image width: {}, height: {}", width, height);

    let rgba_image = image.to_rgba8();
    let bytes = rgba_image.as_raw();

    for i in (0..bytes.len()).step_by(4) {
        let r = bytes[i] as f64;
        let g = bytes[i + 1] as f64;
        let b = bytes[i + 2] as f64;
        let a = bytes[i + 3];
        let pixel_index = i / 4;

        let brightness = (0.299 * r) + (0.587 * g) + (0.114 * g);
        println!(
            "Pixel {}: R={}, G={}, B={}, A={}, Brightness: {}" ,
            pixel_index, r, g, b, a, brightness
        );
    }
}

fn brightness_to_char(brightness: f64) -> char {
    let ascii = ['@', '#', '%', '*', '+', '=', '-', ':', '.'];
    let index = ((brightness / 255.0) * (ascii.len() - 1) as f64).round() as usize;
    ascii[index]
}
