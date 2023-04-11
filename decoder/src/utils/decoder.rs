use std::fs::{self, DirEntry};

use image::{DynamicImage, GenericImageView, Rgba};

fn decode_pixel_hexa(pixel: Rgba<u8>) -> u32 {
    let hex = rbg_to_hex(pixel);
    let value_without_filled = hex.replace("f", "");

    let value = value_without_filled.parse::<u32>().unwrap();

    value
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

pub struct DecodedVideo {
    images: Vec<String>,
}

impl DecodedVideo {
    pub fn new(directory: Vec<DirEntry>) -> DecodedVideo {
        let images = directory
            .iter()
            .map(|entry| String::from(entry.path().to_str().unwrap()))
            .collect();

        DecodedVideo { images }
    }

    fn get_information_image(&self) -> DynamicImage {
        image::open(self.images[0].as_str()).unwrap()
    }

    fn get_number_of_pixels_of_the_title(&self) -> u8 {
        let image = self.get_information_image();
        let number_of_chars_pixel = image.get_pixel(3, 0);

        let number_of_chars = Self::get_number_of_chars_of_the_title(number_of_chars_pixel);

        let number_of_pixels_to_take = number_of_chars * 2;

        // This is the number of valid chars on the last pixel
        let last_pixel_chars = number_of_pixels_to_take % 6;

        let mut number_of_pixels_of_the_title = number_of_pixels_to_take / 6;

        if last_pixel_chars != 0 {
            number_of_pixels_of_the_title += 1;
        }

        number_of_pixels_of_the_title
    }

    fn get_number_of_chars_of_the_title(pixel: Rgba<u8>) -> u8 {
        pixel.0[0]
    }

    pub fn get_images_dimensions(&self) -> (u32, u32) {
        let info_image = image::open(self.images[0].as_str()).unwrap();
        info_image.dimensions()
    }

    pub fn get_file_title(&self) -> String {
        let info_image = self.get_information_image();
        let number_of_chars_pixel = info_image.get_pixel(3, 0);

        let number_of_chars = Self::get_number_of_chars_of_the_title(number_of_chars_pixel);

        // This is the number of chars in hex
        let number_of_pixels_to_take = number_of_chars * 2;

        // This is the number of valid chars on the last pixel
        let last_pixel_chars = number_of_pixels_to_take % 6;

        let number_of_pixels_of_the_title = self.get_number_of_pixels_of_the_title();

        let mut title_hexa_value = String::from("");

        for pixel_index in 4..(4 + number_of_pixels_of_the_title) {
            let title_pixel = info_image.get_pixel(pixel_index as u32, 0);

            let hex = rbg_to_hex(title_pixel);
            title_hexa_value.push_str(&hex);
        }

        let mut number_of_chars_to_remove = 0;
        if last_pixel_chars != 0 {
            number_of_chars_to_remove = 6 - last_pixel_chars;
        }

        title_hexa_value.truncate(title_hexa_value.len() - number_of_chars_to_remove as usize);
        let title = hex::decode(&title_hexa_value).unwrap();

        String::from_utf8(title).unwrap()
    }

    fn get_colors_on_last_pixel(&self) -> u8 {
        let info_image = self.get_information_image();
        let pixel = info_image.get_pixel(0, 0);
        pixel.0[0]
    }

    fn get_valid_height_on_last_image(&self) -> u32 {
        let info_image = self.get_information_image();
        let pixel = info_image.get_pixel(1, 0);
        decode_pixel_hexa(pixel)
    }

    fn get_number_of_pixels_on_last_line(&self) -> u32 {
        let info_image = self.get_information_image();
        let pixel = info_image.get_pixel(2, 0);
        decode_pixel_hexa(pixel)
    }

    fn collect_all_pixels(&self) -> Vec<(u8, u8, u8)> {
        let mut all_data = vec![];
        for image_path in &self.images {
            let image = image::open(image_path).unwrap();

            let (w, h) = image.dimensions();

            for y in 0..h {
                for x in 0..w {
                    let pixel = image.get_pixel(x, y);
                    let rgb = pixel.0;
                    all_data.push((rgb[0], rgb[1], rgb[2]))
                }
            }
        }

        all_data
    }

    pub fn get_relevent_data(&self) -> Vec<u8> {
        let mut all_pixels = self.collect_all_pixels();

        let (width, height) = self.get_images_dimensions();

        let last_line_number_of_pixels = self.get_number_of_pixels_on_last_line();

        let valid_height_on_last_image = self.get_valid_height_on_last_image();

        let color_on_last_pixel = self.get_colors_on_last_pixel();

        let number_of_pixels_to_remove_to_get_valid_height =
            (height - valid_height_on_last_image) * width;

        let new_length = all_pixels.len() - number_of_pixels_to_remove_to_get_valid_height as usize;
        all_pixels.truncate(new_length);

        let number_of_pixels_to_remove_to_get_valid_last_line = width - last_line_number_of_pixels;
        let new_length =
            all_pixels.len() - number_of_pixels_to_remove_to_get_valid_last_line as usize;
        all_pixels.truncate(new_length);

        let information_pixels = 4 + self.get_number_of_pixels_of_the_title();

        let all_pixels = all_pixels.split_off(information_pixels as usize);

        let mut flat_pixels: Vec<u8> = all_pixels
            .into_iter()
            .flat_map(|(a, b, c)| vec![a, b, c])
            .collect();

        let last_len_with_color = flat_pixels.len() - color_on_last_pixel as usize;
        flat_pixels.truncate(last_len_with_color);

        flat_pixels
    }
}
