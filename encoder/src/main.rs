use image::RgbaImage;
use tokio::fs;

const MAX_HEIGHT: i32 = 1920;
const MAX_WIDTH: i32 = 1080;

#[tokio::main]
async fn main() {
    let mut file = fs::read("file.pdf").await.unwrap();

    let len = file.len();

    let last_value_rest: u8 = (len % 3).try_into().unwrap();

    let first_pixel: &[u8] = &[last_value_rest, last_value_rest, last_value_rest];

    let len = file.len();
    let mut chunks = len / 3;
    if len % 3 != 0 {
        chunks += 1;
    }

    file.resize(chunks * 3, 0);
    let chunks = file.chunks_mut(3);

    let numbers_of_pixels = chunks.len();
    println!("pixels: {}", numbers_of_pixels);

    let (dims, rest) = find_integer_dimensions(numbers_of_pixels as i32);

    println!("w: {}, h: {}", dims, dims);

    let image = RgbaImage::new(dims, dims);

    // image.save("test.png").unwrap();

    // for chunk in chunks {
    //     println!("{:?}", chunk);
    // }

    // Group by 3 the u8 from the Vec to create rgb colors
    // We will need a way to tell the decoder how much pixels are valid if there is not enough values to finish the screen
    // We will need an identifier to be able to tell the decoder how many bytes are actually valid if there is not enough data to build the last RBG color
}

fn find_integer_dimensions(area: i32) -> (u32, u32) {
    let dims = (area as f64).ceil().floor() as u32;
    let rest = area as u32 % (dims);

    (dims, rest)
}
