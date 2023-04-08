use std::slice::ChunksMut;

use super::{
    constantes::{self, PixelType},
    functions,
};

pub fn get_valid_filename(filename: &str) -> String {
    let mut formated_filename = String::from(filename);

    if formated_filename.len() > 254 {
        formated_filename = formated_filename.split_off(formated_filename.len() - 254);
    }

    formated_filename
}

pub fn get_pixel_number_four(valid_filename: &String) -> PixelType {
    let valid_filename_length = valid_filename.len();

    (valid_filename_length as u8, 0, 0)
}

pub fn get_pixels_of_the_title(title_rgb_value: String) -> Vec<PixelType> {
    let mut chunked_title = title_rgb_value
        .split("")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let len = chunked_title.len();
    let mut chunks_title = len / 6;
    if len % 6 != 0 {
        chunks_title += 1;
    }
    chunked_title.resize(chunks_title * 6, "f");
    let chunked_title = chunked_title.chunks_mut(6);

    let mut title_pixels: Vec<(u8, u8, u8)> = Vec::new();

    chunked_title.for_each(|t| {
        let rgb = functions::hex_to_rgb(&t.join("")).unwrap();
        title_pixels.push(rgb)
    });

    title_pixels
}

pub fn get_pixel_number_two(height: i32) -> PixelType {
    let second_pixel = height % constantes::MAX_HEIGHT;
    let second_pixel = functions::get_hex_string(second_pixel);
    functions::hex_to_rgb(&second_pixel).unwrap()
}
pub fn get_pixel_number_three(last_line_number_of_pixels: i32) -> PixelType {
    let third_pixel = functions::get_hex_string(last_line_number_of_pixels);
    functions::hex_to_rgb(&third_pixel).unwrap()
}

pub fn find_integer_dimensions(numbers_of_pixels: i32) -> (i32, i32, i32) {
    // let dims = (area as f64).sqrt().ceil().floor() as u32;
    let width = constantes::MAX_WIDTH;
    let mut full_number_of_pixel_lines = numbers_of_pixels / width;
    let last_line_number_of_pixels = numbers_of_pixels % width;

    if last_line_number_of_pixels != 0 {
        full_number_of_pixel_lines += 1;
    }

    (
        width,
        full_number_of_pixel_lines,
        last_line_number_of_pixels,
    )
}

pub fn get_the_number_of_images_to_create(full_height_of_pixels: f64) -> i32 {
    (full_height_of_pixels / constantes::MAX_HEIGHT as f64).ceil() as i32
}

pub fn get_all_pixels(
    first_pixel: PixelType,
    second_pixel: PixelType,
    third_pixel: PixelType,
    fourth_pixel: PixelType,
    title_pixels: Vec<PixelType>,
    file_pixels: ChunksMut<u8>,
) -> Vec<(u8, u8, u8)> {
    let mut all_pixels: Vec<PixelType> = [first_pixel, second_pixel, third_pixel, fourth_pixel]
        .into_iter()
        .collect::<Vec<_>>();

    title_pixels
        .iter()
        .for_each(|px| all_pixels.push((px.0, px.1, px.2)));

    file_pixels.for_each(|chunk| all_pixels.push((chunk[0], chunk[1], chunk[2])));

    all_pixels
}

pub fn resize_file_to_be_chunked(file: &mut Vec<u8>) -> ChunksMut<u8> {
    let len = file.len();

    let mut chunks = len / 3;
    if len % 3 != 0 {
        chunks += 1;
    }

    file.resize(chunks * 3, 0);

    let test = file.chunks_mut(3);
    test
}
