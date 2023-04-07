use image::{DynamicImage, GenericImageView, Rgba};

// First pixel is the number of values valid on the last chunk for exemple if the last chunk is  [12,34] the first pixel value would be [2,2,2]
// This function should return 2
pub fn get_last_pixel_number_relevent_colors(image: &DynamicImage) -> u8 {
    let pixel = image.get_pixel(0, 0);
    pixel.0[0]
}

fn decode_pixel_hexa(pixel: Rgba<u8>) -> u8 {
    let hex = rbg_to_hex(pixel);
    let value_without_filled = hex.replace("f", "");

    let value = value_without_filled.parse::<u8>().unwrap();

    value
}

pub fn get_the_valid_heigt_on_last_image(image: &DynamicImage) -> u8 {
    let pixel = image.get_pixel(1, 0);
    decode_pixel_hexa(pixel)
}

pub fn number_of_pixels_on_last_line(image: &DynamicImage) -> u8 {
    let pixel = image.get_pixel(2, 0);
    decode_pixel_hexa(pixel)
}

pub fn get_file_name(image: &DynamicImage) -> String {
    let number_of_chars_pixel = image.get_pixel(3, 0);

    String::from("")
}

fn get_number_of_chars_of_the_title(pixel: Rgba<u8>) -> u8 {
    0
}

fn rbg_to_hex(rgba: Rgba<u8>) -> String {
    let val = rgba.0;
    let r = val[0];
    let g = val[1];
    let b = val[2];

    format!("{:x}{:x}{:x}", r, g, b)
}
