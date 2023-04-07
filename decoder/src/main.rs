pub mod utils;

use crate::utils::decoder;

#[tokio::main]
async fn main() {
    let image = image::open("result0.png").unwrap();

    let valid_colors_on_last_pixel = decoder::get_last_pixel_number_relevent_colors(&image);

    let last_valid_height = decoder::get_the_valid_heigt_on_last_image(&image);
    let number_of_pixels_on_last_line = decoder::number_of_pixels_on_last_line(&image);

    let filename = decoder::get_file_name(&image);

    let data = decoder::get_data_of_the_image(
        &image,
        number_of_pixels_on_last_line,
        valid_colors_on_last_pixel,
    );

    decoder::write_file(data, filename.clone());
    println!("pixel: {:?}", valid_colors_on_last_pixel);
    println!("last_valid_height: {:?}", last_valid_height);
    println!(
        "number_of_pixels_on_last_line: {:?}",
        number_of_pixels_on_last_line
    );
    println!("filename: {:?}", filename);
}
