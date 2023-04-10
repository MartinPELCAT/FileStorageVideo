pub mod utils;

use futures::future::join_all;
use image::{Rgb, RgbImage};
use tokio::fs;

use crate::utils::{constantes, encoder, functions};

#[tokio::main]
async fn main() {
    let filename = "file.mp3";

    let mut file = fs::read(filename).await.unwrap();

    let len = file.len();

    let last_pixel_number_of_valid_colors: u8 = (len % 3).try_into().unwrap();

    let file_pixels = encoder::resize_file_to_be_chunked(&mut file);

    let valid_filename = encoder::get_valid_filename(filename);

    let pixel_number_4 = encoder::get_pixel_number_four(&valid_filename);

    let title_rgb_value = functions::encode_str_to_hex(valid_filename.as_str());

    let title_pixels = encoder::get_pixels_of_the_title(title_rgb_value);

    let first_pixel = (
        last_pixel_number_of_valid_colors,
        last_pixel_number_of_valid_colors,
        last_pixel_number_of_valid_colors,
    );

    let numbers_of_pixels = file_pixels.len() + 4 + title_pixels.len();

    let (_w, h, last_line_number_of_pixels) =
        encoder::find_integer_dimensions(numbers_of_pixels as i32);

    println!("{h} {last_line_number_of_pixels}");
    let second_pixel = encoder::get_pixel_number_two(h);

    let third_pixel = encoder::get_pixel_number_three(last_line_number_of_pixels);
    let number_of_images_to_create = encoder::get_the_number_of_images_to_create(h as f64);

    let all_pixels = encoder::get_all_pixels(
        first_pixel,
        second_pixel,
        third_pixel,
        pixel_number_4,
        title_pixels,
        file_pixels,
    );

    let mut current_video_height = h;
    if number_of_images_to_create > 1 {
        current_video_height = constantes::MAX_HEIGHT;
    }

    println!("{}", number_of_images_to_create);

    let mut futures = vec![];

    for image_number in 0..number_of_images_to_create {
        let all_pixels = all_pixels.clone();
        let current_video_height = current_video_height;
        let index = image_number;

        let handle = tokio::task::spawn(async move {
            let mut image =
                RgbImage::new(constantes::MAX_WIDTH as u32, current_video_height as u32);

            for y in 0..current_video_height {
                for x in 0..constantes::MAX_WIDTH {
                    let index = (current_video_height * constantes::MAX_WIDTH * index)
                        + (x + (constantes::MAX_WIDTH * y));

                    let pix = image.get_pixel_mut(x as u32, y as u32);

                    let pixel_to_draw = all_pixels.get(index as usize);

                    if let Some(pixel) = pixel_to_draw {
                        *pix = Rgb([pixel.0, pixel.1, pixel.2]);
                    } else {
                        *pix = Rgb([0, 0, 0]);
                    }
                }
            }

            let path = format!("result{:03}.png", index);

            image.save(path).unwrap();
        });

        futures.push(handle);
    }

    join_all(futures).await;
}
