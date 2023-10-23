use std::ffi::OsStr;
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};

pub fn download_video(webpage_url: &str, output_format: &str, dir: &str, thumbnail: bool) {
    let mut binding = Command::new("yt-dlp");
    let command = binding
        .arg(webpage_url)
        .args(["-o", output_format])
        .args(["--paths", dir])
        .arg("--extract-audio")
        .args(["--audio-format", "mp3"]);

    if thumbnail {
        command.arg("--write-thumbnail");
    }
    command.output().expect("download command failed");
}

pub fn convert_jpg(path: &PathBuf) -> PathBuf {
    let new_path = path.to_string_lossy().replace(path.extension().unwrap().to_str().unwrap(), "jpg");
    Command::new("ffmpeg")
        .arg("-i")
        .arg(&path)
        .arg(&new_path)
        .output()
        .expect("ffmpeg image conversion failed");

    return PathBuf::from(new_path);
}

pub fn cleanup(dir: &PathBuf) {
    // remove image files and full videos
    for entry in fs::read_dir(dir).expect("read_dir failed") {
        let entry = entry.expect("failed to get entry");
        if entry.file_name().to_string_lossy().contains("---FULL") {
            let _ = std::fs::remove_file(&entry.path());
            continue;
        }
        if let Some(extension) = Path::new(&entry.path()).extension().and_then(OsStr::to_str) {
            match extension {
                "jpg" => { let _ = std::fs::remove_file(&entry.path()); },
                "jpeg" => { let _ = std::fs::remove_file(&entry.path()); },
                "png" => { let _ = std::fs::remove_file(&entry.path()); },
                "webp" => { let _ = std::fs::remove_file(&entry.path()); },
                _ => {}
            }
        }
    }
}
