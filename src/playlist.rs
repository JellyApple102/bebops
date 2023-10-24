use eframe::egui;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use crate::{UrlInfo, Single, Renderable, Downloadable};
use crate::utils;

extern crate sanitize_filename;

#[derive(Default)]
pub struct Playlist {
    pub playlist_title: String,
    pub songs: Vec<Single>,

    pub use_thumbnail: bool,
    pub cover_path: Option<PathBuf>
}

impl From<Vec<UrlInfo>> for Playlist {
    fn from(urls: Vec<UrlInfo>) -> Self {
        let mut playlist = Playlist::default();
        playlist.use_thumbnail = true;
        if let Some(title) = &urls[0].playlist {
            playlist.playlist_title = title.to_string();
        } else {
            playlist.playlist_title = "Playlist".to_string();
        }
        for url in urls {
            let song = Single::from(url);
            playlist.songs.push(song);
        }
        return playlist
    }
}

impl Renderable for Playlist {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(&mut self.playlist_title);
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.use_thumbnail, "Use Thumbnail");

            if !self.use_thumbnail {
                if ui.button("Pick Image").clicked() {
                    let fd = rfd::FileDialog::new()
                        .add_filter("image", &["png", "jpg", "jpeg", "webp"]);

                    if let Some(path) = fd.pick_file() {
                        self.cover_path = Some(path);
                    }
                }

                if let Some(path) = &self.cover_path {
                    ui.label("Picked:");
                    ui.monospace(path.to_string_lossy());
                }
            } else {
                self.cover_path = None
            }

            if ui.button("Apply").clicked() {
                for song in &mut self.songs {
                    song.use_thumbnail = self.use_thumbnail;
                    if let Some(path) = &self.cover_path {
                        song.cover_path = Some(path.to_path_buf());
                    } else {
                        song.cover_path = None;
                    }
                }
            }
        });
        ui.separator();
        for song in &mut self.songs {
            song.render(ui);
            ui.separator();
        }
    }
}

impl Downloadable for Playlist {
    fn download(&self, base_dir: &PathBuf) {
        let download_dir = base_dir.join("playlists").join(sanitize_filename::sanitize(&self.playlist_title));
        let mut file_string = String::default();

        for song in &self.songs {
            let output_format = format!("{}---{}.%(ext)s", sanitize_filename::sanitize(&song.track), sanitize_filename::sanitize(&song.artist));
            utils::download_video(&song.webpage_url, &output_format, download_dir.to_str().unwrap(), song.use_thumbnail);

            let mp3_name = output_format.replace("%(ext)s", "mp3");
            let mp3_path = &download_dir.join(mp3_name);

            file_string.push_str(mp3_path.to_str().unwrap());
            file_string.push('\n');

            if let Some(path) = &song.cover_path {
                let cover_name = path.file_name().unwrap();
                let new_path = &download_dir.join(cover_name);
                let _ = std::fs::copy(path, new_path);

                let cover_path = utils::convert_jpg(&new_path);
                song.tag(mp3_path, &cover_path, None);
            } else if song.use_thumbnail {
                let cover_name = output_format.replace("%(ext)s", "webp");
                let cover_path = &download_dir.join(cover_name);
                let cover_path = utils::convert_jpg(&cover_path);
                song.tag(mp3_path, &cover_path, None);
            }
        }

        let file_path = download_dir.join(format!("{}.m3u8", sanitize_filename::sanitize(&self.playlist_title)));
        let mut file = File::create(file_path).unwrap();
        let _ = file.write_all(file_string.as_bytes());
        utils::cleanup(&download_dir);
    }
}
