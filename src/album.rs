use eframe::egui;
use std::path::PathBuf;
use crate::{UrlInfo, Single, Renderable, Downloadable};
use crate::utils;

#[derive(Default)]
pub struct Album {
    pub album_title: String,
    pub album_artist: String,
    pub songs: Vec<Single>,

    pub use_thumbnail: bool,
    pub cover_path: Option<PathBuf>
}

impl From<Vec<UrlInfo>> for Album {
    fn from(urls: Vec<UrlInfo>) -> Self {
        let mut album = Album::default();
        album.use_thumbnail = true;
        if let Some(title) = &urls[0].playlist {
            album.album_title = title.to_string();
        } else {
            album.album_title = "Album".to_string();
        }
        album.album_artist = urls[0].artist.to_string();
        for url in urls {
            let song = Single::from(url);
            album.songs.push(song);
        }
        return album
    }
}

impl Renderable for Album {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Album Title");
            ui.text_edit_singleline(&mut self.album_title);
            if ui.button("Apply").clicked() {
                for song in &mut self.songs {
                    song.album = self.album_title.clone();
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("Album Artist");
            ui.text_edit_singleline(&mut self.album_artist);
            if ui.button("Apply").clicked() {
                for song in &mut self.songs {
                    song.artist = self.album_artist.clone();
                }
            }
        });
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

impl Downloadable for Album {
    fn download(&self, base_dir: &PathBuf) {
        let download_dir = base_dir.join("albums").join(&self.album_title);
        for (i, song) in self.songs.iter().enumerate() {
            let output_format = format!("{}---{}.%(ext)s", &song.track, &song.artist);
            utils::download_video(&song.webpage_url, &output_format, download_dir.to_str().unwrap(), song.use_thumbnail);

            let mp3_name = output_format.replace("%(ext)s", "mp3");
            let mp3_path = &download_dir.join(mp3_name);

            if let Some(path) = &song.cover_path {
                let cover_name = path.file_name().unwrap();
                let new_path = &download_dir.join(cover_name);
                let _ = std::fs::copy(path, new_path);

                let cover_path = utils::convert_jpg(&new_path);
                song.tag(mp3_path, &cover_path, Some(i + 1));
            } else if song.use_thumbnail {
                let cover_name = output_format.replace("%(ext)s", "webp");
                let cover_path = &download_dir.join(cover_name);
                let cover_path = utils::convert_jpg(&cover_path);
                song.tag(mp3_path, &cover_path, Some(i + 1));
            }
        }

        utils::cleanup(&download_dir);
    }
}
