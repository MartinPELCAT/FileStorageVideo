pub mod utils;

use image::{Rgb, RgbImage};
use tokio::fs;

use crate::utils::functions;

const MAX_WIDTH: i32 = 1920;
const MAX_HEIGHT: i32 = 1080;

#[tokio::main]
async fn main() {
    let filename = "file.pdf";

    let mut file = fs::read(filename).await.unwrap();

    let len = file.len();

    let last_value_rest: u8 = (len % 3).try_into().unwrap();

    let len = file.len();
    let mut chunks = len / 3;
    if len % 3 != 0 {
        chunks += 1;
    }

    file.resize(chunks * 3, 0);
    let chunks = file.chunks_mut(3);

    // First pixel is the number of values valid on the last chunk for exemple if the last chunk is  [12,34] the first pixel value would be [2,2,2]
    // Second pixel is the full height of the valid pixels on the last page so max is MAX_HEIGHT
    // Third pixel is the Number of pixels valid on the last line the format being "85ffff" = 85 pixels
    // The pixel number 4 is the number of char on the filename and the valid char on the last pixels before the data exemple
    // : "file.pdf" => "66696c652e706466" => split by 6 => ["66696c","652e70","6466"] => last pixel = (8,0,4) => like (filename.len(), 0, lastColorDigitValidChars)
    // So we know that ( `8` * 2 = 16) and 16 % 6 = `4` => 4 is the lastColorDigitValidChars
    // On the decoder, we will be able to know that the next (`8` * 2 = 16) => 16 / 6 = 2.6666 => ceil(2.6666) => 3 so the next 3 pixels are the filename
    // The filename.len() must be < 255 the filename will be cropped from the start to avoid losing the extension

    let mut formated_filename = String::from(filename);
    if formated_filename.len() > 254 {
        formated_filename = formated_filename.split_off(formated_filename.len() - 254);
    }

    let formated_filename_length = formated_filename.len();

    let last_color_digit_calid_chars = (formated_filename_length * 2) % 6;

    let title_info_pixel: (u8, u8, u8) = (
        formated_filename_length as u8,
        0,
        last_color_digit_calid_chars as u8,
    );

    println!("new name :{}", formated_filename);
    let title_rgb = functions::encode_str_to_hex(formated_filename.as_str());

    println!("title: {} title rgb: {}", filename, title_rgb);
    let mut chunked_title = title_rgb
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

    println!("chunked title: {:?}, {chunks_title}", chunked_title);

    let mut title_pixels: Vec<(u8, u8, u8)> = Vec::new();

    chunked_title.for_each(|t| {
        let rgb = functions::hex_to_rgb(&t.join("")).unwrap();
        title_pixels.push(rgb)
    });

    let first_pixel = (last_value_rest, last_value_rest, last_value_rest);
    println!("first pixel : {:?}", first_pixel);
    let numbers_of_pixels = chunks.len() + 4 + chunks_title;
    println!("pixels: {}", numbers_of_pixels);

    let (w, h, rest) = find_integer_dimensions(numbers_of_pixels as i32);
    let second_pixel = h % MAX_HEIGHT;
    let second_pixel = functions::get_hex_string(second_pixel);
    let second_pixel = functions::hex_to_rgb(&second_pixel).unwrap();
    println!("second pixel: {:?}", second_pixel);

    let third_pixel = functions::get_hex_string(rest);

    println!("w: {}, h: {}, rest: {}", w, h, rest);
    println!("pixel hexa, {}", third_pixel);
    let third_pixel = functions::hex_to_rgb(&third_pixel).unwrap();
    println!("pixel rgb {:?}", third_pixel);

    let number_of_images_to_create = (h as f64 / MAX_HEIGHT as f64).ceil() as i32;

    println!("create {number_of_images_to_create} images");

    let mut all_pixels: Vec<(u8, u8, u8)> =
        [first_pixel, second_pixel, third_pixel, title_info_pixel]
            .into_iter()
            .collect::<Vec<_>>();

    title_pixels
        .iter()
        .for_each(|px| all_pixels.push((px.0, px.1, px.2)));

    chunks.for_each(|chunk| all_pixels.push((chunk[0], chunk[1], chunk[2])));

    // println!("All pixels {:?}", all_pixels);

    let mut current_video_height = h;
    if number_of_images_to_create > 1 {
        current_video_height = MAX_HEIGHT;
    }

    for image_number in 0..number_of_images_to_create {
        let mut image = RgbImage::new(MAX_WIDTH as u32, current_video_height as u32);

        for y in 0..current_video_height {
            for x in 0..MAX_WIDTH {
                let index =
                    (current_video_height * MAX_WIDTH * image_number) + (x + (MAX_WIDTH * y));

                // println!("{index}");

                let pix = image.get_pixel_mut(x as u32, y as u32);

                let pixel_to_draw = all_pixels.get(index as usize);

                if let Some(pixel) = pixel_to_draw {
                    *pix = Rgb([pixel.0, pixel.1, pixel.2]);
                } else {
                    *pix = Rgb([0, 0, 0]);
                }
            }
        }

        let path = format!("result{}.png", image_number);

        image.save(path).unwrap();
    }
}

fn find_integer_dimensions(area: i32) -> (i32, i32, i32) {
    // let dims = (area as f64).sqrt().ceil().floor() as u32;
    let width = MAX_WIDTH;
    let mut height = (area) / width;
    let rest = area % width;

    if rest != 0 {
        height += 1;
    }

    // Rest is the numbers of pixels on the last lines that are importants
    // This cannot be higher than 1920

    // Height can be way to high.
    (width, height, rest)
}
