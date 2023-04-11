pub mod utils;

use std::fs;

use crate::utils::{
    decoder::{self, DecodedVideo},
    video,
};

#[tokio::main]
async fn main() {
    let out_dir_path = "out";
    if !fs::metadata(out_dir_path).is_ok() {
        fs::create_dir(out_dir_path).unwrap();
    } else {
        fs::remove_dir_all(out_dir_path).unwrap();
        fs::create_dir(out_dir_path).unwrap();
    }

    video::extract_video();

    let entries = fs::read_dir(out_dir_path).unwrap();

    let mut files: Vec<_> = entries.filter_map(|file| file.ok()).collect();
    files.sort_by_key(|entry| entry.path());

    let decoded_video = DecodedVideo::new(files);

    let data = decoded_video.get_relevent_data().await;

    decoder::write_file(data, decoded_video.get_file_title());
}
