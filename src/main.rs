use image::{GenericImageView, ImageReader};
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let image = ImageReader::open("images/image_100x100.jpg")
        .expect("Image not found")
        .decode()
        .expect("Failed to decode image");

    let (width, height) = image.dimensions();
    println!("Image width: {}, height: {}", width, height);

    let (terminal_width, terminal_height) = match terminal_size() {
        Some((Width(w), Height(h))) => (w as usize, h as usize),
        None => (80, 24), // default value
    };
    println!("Terminal width: {terminal_width}, height: {terminal_height}");

    let rgba_image = image.to_rgba8();
    let bytes = rgba_image.as_raw();

    for i in (0..bytes.len()).step_by(4) {
        let r = bytes[i] as f64;
        let g = bytes[i + 1] as f64;
        let b = bytes[i + 2] as f64;
        let a = bytes[i + 3] as f64 / 255.0; // normalize alpha
        let pixel_index = i / 4;

        let r_blend = r * a;
        let g_blend = g * a;
        let b_blend = b * a;

        let brightness = 0.299 * r_blend + 0.587 * g_blend + 0.114 * b_blend;
        let char = brightness_to_char(brightness);

        if pixel_index % width as usize == 0 {
            println!();
        }

        print!("{} ", char);
    }

    println!();
}
fn brightness_to_char(brightness: f64) -> char {
    let ascii = ['@', '#', '%', '*', '+', '=', '-', ':', '.'];
    let index = ((brightness / 255.0) * (ascii.len() - 1) as f64).round() as usize;
    ascii[index]
}
