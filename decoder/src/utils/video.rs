use std::process::Command;

pub fn extract_video() {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg("output.avi")
        .arg("-c:v")
        .arg("png")
        .arg("-r")
        .arg("2")
        .arg("out/output%03d.png")
        .output()
        .unwrap();

    if output.stderr.len() > 0 {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    // ffmpeg -i input.mp4 -r 2 out/output%03d.png
}
