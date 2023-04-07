use std::fs;

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

    let number_of_chars = get_number_of_chars_of_the_title(number_of_chars_pixel);

    // This is the number of chars in hex
    let number_of_pixels_to_take = number_of_chars * 2;

    // This is the number of valid chars on the last pixel
    let last_pixel_chars = number_of_pixels_to_take % 6;

    let number_of_pixels_of_the_title = get_number_of_pixels_of_the_title(&image);

    let mut title_hexa_value = String::from("");

    for pixel_index in 4..(4 + number_of_pixels_of_the_title) {
        let title_pixel = image.get_pixel(pixel_index as u32, 0);

        let hex = rbg_to_hex(title_pixel);
        title_hexa_value.push_str(&hex);
    }

    let number_of_chars_to_remove = 6 - last_pixel_chars;

    title_hexa_value.truncate(title_hexa_value.len() - number_of_chars_to_remove as usize);
    let title = hex::decode(&title_hexa_value).unwrap();

    String::from_utf8(title).unwrap()
}

pub fn get_data_of_the_image(
    image: &DynamicImage,
    number_of_pixels_on_last_line: u8,
    valid_colors_on_last_pixel: u8,
) -> Vec<u8> {
    let number_of_pixels_of_the_title = get_number_of_pixels_of_the_title(&image);

    let (width, height) = image.dimensions();

    let mut data = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if y == 0 && x < (number_of_pixels_of_the_title + 4).into() {
                continue;
            }

            if y == height - 1 && x > (number_of_pixels_on_last_line - 1).into() {
                continue;
            }

            let pix = image.get_pixel(x, y);
            let rgb = pix.0;

            if y == height - 1 && x == (number_of_pixels_on_last_line - 1).into() {
                println!("last pixel valid : {:?}", pix);

                for i in 0..(valid_colors_on_last_pixel as usize) {
                    data.push(rgb[i]);
                }

                continue;
            }
            data.push(rgb[0]);
            data.push(rgb[1]);
            data.push(rgb[2]);
        }
    }

    data
}

fn get_number_of_chars_of_the_title(pixel: Rgba<u8>) -> u8 {
    pixel.0[0]
}
fn get_number_of_pixels_of_the_title(image: &DynamicImage) -> u8 {
    let number_of_chars_pixel = image.get_pixel(3, 0);

    let number_of_chars = get_number_of_chars_of_the_title(number_of_chars_pixel);

    let number_of_pixels_to_take = number_of_chars * 2;

    // This is the number of valid chars on the last pixel
    let last_pixel_chars = number_of_pixels_to_take % 6;

    let mut number_of_pixels_of_the_title = number_of_pixels_to_take / 6;

    if last_pixel_chars != 0 {
        number_of_pixels_of_the_title += 1;
    }

    number_of_pixels_of_the_title
}

fn rbg_to_hex(rgba: Rgba<u8>) -> String {
    let val = rgba.0;
    let r = val[0];
    let g = val[1];
    let b = val[2];

    format!("{:x}{:x}{:x}", r, g, b)
}

pub fn write_file(data: Vec<u8>, filename: String) {
    fs::write(filename, data).unwrap();
}
