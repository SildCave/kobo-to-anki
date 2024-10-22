#![windows_subsystem = "windows"]

use std::{path::PathBuf, sync::{Arc, Mutex}};

use anki_bridge::prelude::CardsInfoResponse;
use eframe::egui::{self, IconData};
use egui_file_dialog::FileDialog;
use tokio::runtime;

mod layout;
mod events;
mod kobo;
mod anki;
mod cards;
mod dictionary;

struct Channels {
    anki_connection_status_rc: Option<tokio::sync::mpsc::Receiver<anki::AnkiConnectionStatus>>,
}

struct AppState {
    file_dialog: FileDialog,
    async_rt: runtime::Runtime,
    kobo_path: Option<PathBuf>,
    invalid_kobo_path: bool,
    anki_client: Arc<anki::AnkiClient<'static>>,
    anki_connection_status: anki::AnkiConnectionStatus,
    channels: Channels,
    custom_path: bool,
    first_attempt_at_connecting_to_anki: bool,
    last_connection_attempt_time: std::time::Instant,
    prepared_words_from_kobo: Option<Vec<String>>,
    prepared_words_from_anki: Option<Vec<String>>,
    raw_cards_from_anki: Option<Vec<CardsInfoResponse>>,
    selected_deck_name: Option<String>,
    deck_names: Option<Vec<String>>,
    sync_started: bool,
    sync_progress: Arc<Mutex<f32>>,
    sync_started_at: Option<std::time::Instant>,
    words_to_add: Option<Vec<String>>,
    server_connection_status: Arc<Mutex<bool>>,
    processed_words: Arc<Mutex<u32>>,
    words_with_no_definitions: Arc<Mutex<u32>>,
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        layout::setup_custom_fonts(&cc.egui_ctx);
        Self {
            file_dialog: FileDialog::new(),
            async_rt: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            kobo_path: None,
            invalid_kobo_path: false,
            anki_client: Arc::new(anki::AnkiClient::new()),
            anki_connection_status: anki::AnkiConnectionStatus::Disconnected,
            channels: Channels {
                anki_connection_status_rc: None,
            },
            custom_path: false,
            first_attempt_at_connecting_to_anki: true,
            last_connection_attempt_time: std::time::Instant::now() - std::time::Duration::from_secs(5),
            prepared_words_from_kobo: None,
            prepared_words_from_anki: None,
            selected_deck_name: None,
            deck_names: None,
            raw_cards_from_anki: None,
            sync_started: false,
            sync_progress: Arc::new(Mutex::new(0.0)),
            sync_started_at: None,
            words_to_add: None,
            server_connection_status: Arc::new(Mutex::new(true)),
            processed_words: Arc::new(Mutex::new(0)),
            words_with_no_definitions: Arc::new(Mutex::new(0)),

        }
    }
}


impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
        events::update_ui(self, ctx);
    }
}


fn load_icon() -> IconData {
    let icon = include_bytes!(
        "../assets/icons/icon.png"
    );
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
fn main() -> eframe::Result<()> {
    env_logger::init();
    let icon = Arc::new(load_icon());
    let mut native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    };
    native_options.viewport.min_inner_size = Some(egui::vec2(800.0, 600.0));

    eframe::run_native(
        "Kobo To Anki Sync Tool",
        native_options,
        Box::new(|ctx| Ok(Box::new(AppState::new(ctx)))),
    )
}



