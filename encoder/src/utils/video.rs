use std::process::Command;

pub fn create_video() {
    let output = Command::new("ffmpeg")
        .arg("-y")
        .arg("-framerate")
        .arg("2")
        .arg("-i")
        .arg("out/result%03d.png")
        .arg("-c:v")
        .arg("ffv1")
        .arg("output.avi")
        .output()
        .unwrap();

    if output.stderr.len() > 0 {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
