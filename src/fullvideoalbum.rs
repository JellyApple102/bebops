use eframe::egui;
use std::path::PathBuf;
use std::process::Command;
use crate::{UrlInfo, Single, Chapter, Renderable, Downloadable};
use crate::utils;

#[derive(Default)]
pub struct FullVideoAlbum {
    pub webpage_url: String,
    pub description: String,
    pub use_thumbnail: bool,
    pub cover_path: Option<PathBuf>,

    pub album_title: String,
    pub album_artist: String,
    pub songs: Vec<Single>,
    pub chapters: Vec<Chapter>,

    pub marked: Option<usize>
}

impl From<UrlInfo> for FullVideoAlbum {
    fn from(url: UrlInfo) -> Self {
        let mut fv_album = FullVideoAlbum::default();
        fv_album.webpage_url = url.webpage_url;
        fv_album.description = url.description;
        fv_album.use_thumbnail = true;

        fv_album.album_title = url.title;
        fv_album.album_artist = url.uploader;

        if let Some(chapters) = url.chapters {
            fv_album.chapters = chapters;
        } else {
            fv_album.chapters = Vec::new();
        }

        fv_album.songs = Vec::new();
        for chapter in &fv_album.chapters {
            let mut song = Single::default();
            song.use_thumbnail = true;
            song.track = chapter.title.to_string();
            song.artist = fv_album.album_artist.clone();
            fv_album.songs.push(song);
        }

        return fv_album
    }
}

impl Renderable for FullVideoAlbum {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Album Title");
            ui.text_edit_singleline(&mut self.album_title);
            if ui.button("Apply").clicked() {
                for song in &mut self.songs {
                    song.album = self.album_title.to_string();
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("Album Artist");
            ui.text_edit_singleline(&mut self.album_artist);
            if ui.button("Apply").clicked() {
                for song in &mut self.songs {
                    song.artist = self.album_artist.to_string();
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
        for (i, song) in &mut self.songs.iter_mut().enumerate() {
            // render songs slightly differently
            ui.horizontal(|ui| {
                ui.label("Title");
                ui.text_edit_singleline(&mut song.track);
            });
            ui.horizontal(|ui| {
                ui.label("Artist");
                ui.text_edit_singleline(&mut song.artist);
            });
            ui.horizontal(|ui| {
                ui.label("Album");
                ui.text_edit_singleline(&mut song.album);
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut song.use_thumbnail, "Use Thumbnail");

                if !song.use_thumbnail {
                    if ui.button("Pick Image").clicked() {
                        let fd = rfd::FileDialog::new()
                            .add_filter("image", &["png", "jpg", "jpeg", "webp"]);

                        if let Some(path) = fd.pick_file() {
                            song.cover_path = Some(path);
                        }
                    }

                    if let Some(path) = &song.cover_path {
                        ui.label("Picked:");
                        ui.monospace(path.to_string_lossy());
                    }
                } else {
                    song.cover_path = None
                }
            });

            let chapter = self.chapters.get_mut(i).unwrap();
            ui.add(egui::DragValue::new(&mut chapter.start_time)
                .clamp_range(0..=((60 * 60 * 24) - 1))
                .custom_formatter(|n, _| {
                    let n = n as i32;
                    let hours = n / (60 * 60);
                    let mins = (n / 60) % 60;
                    let secs = n % 60;
                    return format!("{hours:02}:{mins:02}:{secs:02}")
                })
                .custom_parser(|s| {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() == 3 {
                        parts[0].parse::<i32>().and_then(|h| {
                            parts[1].parse::<i32>().and_then(|m| {
                                parts[2].parse::<i32>().map(|s| {
                                    return ((h * 60 * 60) + (m * 60) + s) as f64
                                })
                            })
                        })
                        .ok()
                    } else {
                        return None
                    }
                })
            );

            ui.add(egui::DragValue::new(&mut chapter.end_time)
                .clamp_range(0..=((60 * 60 * 24) - 1))
                .custom_formatter(|n, _| {
                    let n = n as i32;
                    let hours = n / (60 * 60);
                    let mins = (n / 60) % 60;
                    let secs = n % 60;
                    return format!("{hours:02}:{mins:02}:{secs:02}")
                })
                .custom_parser(|s| {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() == 3 {
                        parts[0].parse::<i32>().and_then(|h| {
                            parts[1].parse::<i32>().and_then(|m| {
                                parts[2].parse::<i32>().map(|s| {
                                    return ((h * 60 * 60) + (m * 60) + s) as f64
                                })
                            })
                        })
                        .ok()
                    } else {
                        return None
                    }
                })
            );

            if ui.button("Remove Chapter").clicked() {
                self.marked = Some(i);
            }

            ui.separator();
        }

        if ui.button("Add Chapter").clicked() {
            self.add_chapter();
        }

        ui.collapsing("Description", |ui| { ui.label(&self.description); });

        self.remove_marked();
    }
}

impl Downloadable for FullVideoAlbum {
    fn download(&self, base_dir: &PathBuf) {
        let download_dir = base_dir.join("albums").join(&self.album_title);
        let output_format = format!("{}---FULL.%(ext)s", &self.album_title);
        utils::download_video(&self.webpage_url, &output_format, download_dir.to_str().unwrap(), self.use_thumbnail);

        let full_mp3_name = output_format.replace("%(ext)s", "mp3");
        let full_mp3_path = download_dir.join(full_mp3_name);
        for (i, song) in self.songs.iter().enumerate() {
            let mp3_name = format!("{}---{}.mp3", &song.track, &song.artist);
            let song_mp3_path = download_dir.join(mp3_name);

            let chapter = self.chapters.get(i).unwrap();
            Command::new("ffmpeg")
                .args(["-ss", &chapter.start_time.to_string()])
                .args(["-to", &chapter.end_time.to_string()])
                .args(["-i", &full_mp3_path.to_str().unwrap()])
                .args(["-c", "copy"])
                .arg(&song_mp3_path)
                .output()
                .expect("ffmpeg split failed");

            if let Some(path) = &song.cover_path {
                let cover_name = path.file_name().unwrap();
                let new_path = &download_dir.join(cover_name);
                let _ = std::fs::copy(path, new_path);

                let cover_path = utils::convert_jpg(&new_path);
                song.tag(&song_mp3_path, &cover_path, Some(i + 1));
            } else if song.use_thumbnail {
                let cover_name = output_format.replace("%(ext)s", "webp");
                let cover_path = &download_dir.join(cover_name);
                let cover_path = utils::convert_jpg(&cover_path);
                song.tag(&song_mp3_path, &cover_path, Some(i + 1));
            }
        }

        utils::cleanup(&download_dir);
    }
}

impl FullVideoAlbum {
    fn add_chapter(&mut self) {
        self.chapters.push(Chapter {
            start_time: 0.0,
            end_time: 0.0,
            title: "New Chapter".to_string()
        });

        let mut song = Single::default();
        song.artist = self.album_artist.to_string();
        song.album = self.album_title.to_string();
        self.songs.push(song);
    }

    fn remove_marked(&mut self) {
        if let Some(i) = self.marked {
            self.chapters.remove(i);
            self.songs.remove(i);
            self.marked = None
        }
    }
}
