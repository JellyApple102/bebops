#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use serde::Deserialize;
use core::f32;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

mod single;
mod playlist;
mod album;
mod fullvideoplaylist;
mod fullvideoalbum;
mod utils;
use single::Single;
use playlist::Playlist;
use album::Album;
use fullvideoplaylist::FullVideoPlaylist;
use fullvideoalbum::FullVideoAlbum;

struct MyApp {
    base_download_dir: PathBuf,
    current_url_string: String,
    current_download_type: DownloadType,
    content: Option<Box<dyn RendDownable>>
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            base_download_dir: PathBuf::default(),
            current_url_string: String::default(),
            current_download_type: DownloadType::default(),
            content: None
        }
    }
}

trait Renderable {
    fn render(&mut self, ui: &mut egui::Ui);
}

trait Downloadable {
    fn download(&self, base_dir: &PathBuf);
}

trait RendDownable: Renderable + Downloadable {}
impl<T> RendDownable for T where T: Renderable + Downloadable {}

#[derive(Default, Debug, PartialEq)]
enum DownloadType {
    #[default]
    Single,
    Playlist,
    Album,
    FullVideoPlaylist,
    FullVideoAlbum
}

#[derive(Deserialize, Default)]
#[allow(dead_code)]
struct UrlInfo {
    webpage_url: String,
    title: String,
    uploader: String,
    thumbnail: String,
    description: String,

    #[serde(default)]
    track: String,
    #[serde(default)]
    artist: String,
    #[serde(default)]
    album: String,

    playlist: Option<String>,
    // playlist_index: Option<u32>,
    chapters: Option<Vec<Chapter>>
}

#[derive(Deserialize)]
pub struct Chapter {
    pub start_time: f32,
    pub end_time: f32,
    pub title: String
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();

        let dir = dirs::audio_dir().unwrap().join("bebops");
        app.update_download_dir(dir);

        return app;
    }

    fn update_download_dir(&mut self, dir: PathBuf) {
        self.base_download_dir = dir;
        fs::create_dir_all(&self.base_download_dir).unwrap();
    }

    fn fetch(&mut self) {
        let command = Command::new("yt-dlp")
            .arg("-j")
            .arg(&self.current_url_string)
            .output()
            .expect("failed to make fetch command");

        let binding = String::from_utf8(command.stdout).unwrap();
        let output = binding.trim();
        let jsons: Vec<&str> = output.split('\n').collect();
        let mut urls: Vec<UrlInfo> = Vec::with_capacity(jsons.len());
        for json in jsons {
            let info: UrlInfo = serde_json::from_str(json).unwrap();
            urls.push(info)
        }

        self.content = match self.current_download_type {
            DownloadType::Single => Some(Box::new(Single::from(urls.swap_remove(0)))),
            DownloadType::Playlist => Some(Box::new(Playlist::from(urls))),
            DownloadType::Album => Some(Box::new(Album::from(urls))),
            DownloadType::FullVideoPlaylist => Some(Box::new(FullVideoPlaylist::from(urls.swap_remove(0)))),
            DownloadType::FullVideoAlbum => Some(Box::new(FullVideoAlbum::from(urls.swap_remove(0)))),
        }
    }

    fn clear_content(&mut self) {
        self.current_url_string = String::default();
        self.content = None;
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let url_text_edit = egui::TextEdit::singleline(&mut self.current_url_string);
                ui.add(url_text_edit);

                egui::ComboBox::from_id_source("Download Type")
                    .selected_text(format!("{:?}", self.current_download_type))
                    .width(120.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.current_download_type, DownloadType::Single, "Single");
                        ui.selectable_value(&mut self.current_download_type, DownloadType::Playlist, "Playlist");
                        ui.selectable_value(&mut self.current_download_type, DownloadType::Album, "Album");
                        ui.selectable_value(&mut self.current_download_type, DownloadType::FullVideoPlaylist, "Full Video Playlist");
                        ui.selectable_value(&mut self.current_download_type, DownloadType::FullVideoAlbum, "Full Video Album");
                    });

                if ui.button("Fetch").clicked() {
                    self.fetch();
                }

                if ui.button("Download").clicked() {
                    if let Some(content) = &mut self.content {
                        content.download(&self.base_download_dir);
                    }
                }

                if ui.button("Clear").clicked() {
                    self.clear_content();
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.hyperlink_to("GitHub", "https://github.com/JellyApple102/BeBops");
            });
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ui.separator();
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                if let Some(content) = &mut self.content {
                    content.render(ui);
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        format!("bebops v{}", VERSION).as_str(),
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc)))
    )
}
