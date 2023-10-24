use eframe::egui;
use id3::{Tag, TagLike, Version};
use id3::frame::{Picture, PictureType};
use std::path::PathBuf;
use crate::{UrlInfo, Renderable, Downloadable};
use crate::utils;

extern crate sanitize_filename;

#[derive(Default)]
pub struct Single {
    pub webpage_url: String,
    pub title: String,
    pub use_thumbnail: bool,
    pub cover_path: Option<PathBuf>,

    pub artist: String,
    pub track: String,
    pub album: String,

    pub description: String,
}

impl From<UrlInfo> for Single {
    fn from(url: UrlInfo) -> Self {
        let mut single = Single::default();
        single.webpage_url = url.webpage_url;
        single.use_thumbnail = true;
        single.title = url.title;
        single.artist = url.artist;
        single.track = url.track;
        single.album = url.album;
        single.description = url.description;

        return single
    }
}

impl Renderable for Single {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.label(&self.title);

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
                    ui.label("Picked");
                    ui.monospace(path.to_string_lossy());
                }
            } else {
                self.cover_path = None;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Title");
            ui.text_edit_singleline(&mut self.track)
        });
        ui.horizontal(|ui| {
            ui.label("Artist");
            ui.text_edit_singleline(&mut self.artist)
        });
        ui.horizontal(|ui| {
            ui.label("Album");
            ui.text_edit_singleline(&mut self.album)
        });
        egui::CollapsingHeader::new("Description")
            .id_source(&self.webpage_url)
            .show(ui, |ui| {
                ui.label(&self.description);
            });
    }
}

impl Downloadable for Single {
    fn download(&self, base_dir: &PathBuf) {
        let download_dir = base_dir.join("singles");
        let output_format = format!("{}---{}.%(ext)s", sanitize_filename::sanitize(&self.track), sanitize_filename::sanitize(&self.artist));
        utils::download_video(&self.webpage_url, &output_format, download_dir.to_str().unwrap(), self.use_thumbnail);

        let mp3_name = output_format.replace("%(ext)s", "mp3");
        let mp3_path = &download_dir.join(mp3_name);

        if let Some(path) = &self.cover_path {
            let cover_name = path.file_name().unwrap();
            let new_path = &download_dir.join(cover_name);
            let _ = std::fs::copy(path, new_path);

            let cover_path = utils::convert_jpg(&new_path);
            self.tag(mp3_path, &cover_path, None);
        } else if self.use_thumbnail {
            let cover_name = output_format.replace("%(ext)s", "webp");
            let cover_path = &download_dir.join(cover_name);
            let cover_path = utils::convert_jpg(&cover_path);
            self.tag(mp3_path, &cover_path, None);
        }

        utils::cleanup(&download_dir);
    }
}

impl Single {
    pub fn tag(&self, mp3_path: &PathBuf, cover_path: &PathBuf, track_no: Option<usize>) {
        let mut tag = Tag::new();

        tag.set_title(&self.track);
        tag.set_artist(&self.artist);
        tag.set_album(&self.album);

        if let Some(n) = track_no {
            tag.set_track(n.try_into().unwrap());
        }

        match std::fs::read(&cover_path) {
            Ok(bytes) => {
                tag.add_frame(Picture{
                    mime_type: "image/jpeg".to_string(),
                    picture_type: PictureType::CoverFront,
                    description: "".to_string(),
                    data: bytes
                });
            },
            _ => { println!("image file") }
        }

        tag.write_to_path(mp3_path, Version::Id3v24).expect("Failed to write tag");
    }
}
