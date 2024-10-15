use std::sync::Arc;

use eframe::egui::{
    self,
    text::LayoutJob,
    Color32, FontFamily,
    FontId, RichText,
    TextFormat
};

use crate::AppState;

pub fn draw_header(
    ui: &mut egui::Ui
) {
    ui.vertical_centered(|ui| {
        ui.heading(RichText::new("Kobo To Anki Sync Tool")
            .color(Color32::WHITE)
            .font(FontId::new(50.0, FontFamily::Monospace))
            .monospace()
        );
    });
}


pub fn draw_kobo_connection_information_message(app_state: &AppState, ui: &mut egui::Ui) {

    ui.vertical_centered(|ui| {
        let text: String;
        if app_state.kobo_path.is_none() {
            text = "Please connect your Kobo eReader, If for some reason it is not detected or you want to select the path to your reader manually, please click the button below.".to_string();
        } else {
            text = "Kobo Reader detected! If for some reason you want to change the path to your reader, please click the button below.".to_string();
        }
        ui.label(RichText::new(text)
            .color(Color32::LIGHT_GRAY)
            .font(FontId::new(20.0, FontFamily::Proportional)));

    });
}

pub fn draw_kobo_connection_status_message_when_no_device_is_detected(app_state: &mut AppState, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("Kobo Reader not detected!")
            .color(Color32::LIGHT_RED)
            .font(FontId::new(20.0, FontFamily::Proportional)));
        draw_file_picker_button(app_state, ui);
    });
}

pub fn draw_kobo_connection_status_message_when_device_is_detected(app_state: &mut AppState, ui: &mut egui::Ui) {
    let mut job = LayoutJob::default();
    job.append(
        "Kobo path: ",
        0.0,
        TextFormat {
            font_id: FontId::new(20.0, FontFamily::Proportional),
            color: Color32::LIGHT_BLUE,
            ..Default::default()
        },
    );
    job.append(
        format!("{}", app_state.kobo_path.as_ref().unwrap().to_str().unwrap()).as_str(),
        0.0,
        TextFormat {
            font_id: FontId::new(20.0, FontFamily::Proportional),
            color: Color32::GREEN,
            italics: false,
            ..Default::default()
        },
    );
    ui.vertical_centered(|ui| {
        ui.label(job);

        draw_file_picker_button(app_state, ui);
    });
}

fn draw_file_picker_button(app_state: &mut AppState, ui: &mut egui::Ui) {
    if ui.button(
        RichText::new("Select/Change Path")
            .color(Color32::LIGHT_BLUE)
            .background_color(Color32::from_rgb(30, 30, 46))
            
            .font(FontId::new(20.0, FontFamily::Proportional))
        ).clicked() {
        // Open the file dialog to select a file.
        app_state.file_dialog.select_directory();
    }
}

pub fn notify_user_about_invalid_kobo_path(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("The path you just selected is not a valid Kobo eReader path!")
            .color(Color32::LIGHT_RED)
            .font(FontId::new(20.0, FontFamily::Proportional)));
    });
}

pub fn display_anki_connection_status_message(app_state: &AppState, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        let text: String;
        let color: Color32;
        match app_state.anki_connection_status {
            crate::anki::AnkiConnectionStatus::Connected => {
                text = "Connected to Anki!".to_string();
                color = Color32::GREEN;
            }
            crate::anki::AnkiConnectionStatus::Connecting => {
                if app_state.first_attempt_at_connecting_to_anki {
                    text = "Connecting to Anki...".to_string();
                    color = Color32::WHITE;
                } else {
                    text = "Could not connect to Anki! Retrying...".to_string();
                    color = Color32::LIGHT_RED;
                }
            }
            crate::anki::AnkiConnectionStatus::Disconnected => {
                text = "Anki disconnected!".to_string();
                color = Color32::LIGHT_RED;
            }
            crate::anki::AnkiConnectionStatus::CouldNotConnect => {
                text = "Could not connect to Anki! Retrying...".to_string();
                color = Color32::LIGHT_RED;
            }
        }
        if (app_state.anki_connection_status == crate::anki::AnkiConnectionStatus::Connecting) || app_state.anki_connection_status == crate::anki::AnkiConnectionStatus::CouldNotConnect {

            ui.vertical_centered_justified(|ui| {
                ui.label(RichText::new(text)
                    .color(color)
                    .font(FontId::new(20.0, FontFamily::Proportional)));
                ui.add(egui::Spinner::new());
            });

        } else {
            ui.label(RichText::new(text)
                .color(color)
                .font(FontId::new(20.0, FontFamily::Proportional)));
        }

    });
}

pub fn draw_horizontal_line(ui: &mut egui::Ui) {
    ui.separator();
}

pub fn draw_anki_connection_guide(ui: &mut egui::Ui) {

    ui.vertical_centered(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("To connect to Anki, please make sure that Anki is running and that")
                .color(Color32::WHITE)
                .font(FontId::new(20.0, FontFamily::Proportional)));
            ui.hyperlink_to(RichText::new("AnkiConnect")
                .color(Color32::LIGHT_BLUE)
                .font(FontId::new(20.0, FontFamily::Proportional)), "https://ankiweb.net/shared/info/2055492159");
            ui.label(RichText::new("is installed.")
                .color(Color32::WHITE)
                .font(FontId::new(20.0, FontFamily::Proportional)));
        });
    });
}

pub fn display_deck_selection_dropdown(app_state: &mut AppState, ui: &mut egui::Ui) {
    // if app_state.selected_deck_name.is_some() {
    //     return;
    // }
    if app_state.deck_names.is_none() {
        let anki_client = app_state.anki_client.clone();
        let decks = app_state.async_rt.block_on(async move {
            return anki_client.get_decks().await;
            //app_state.deck_names = Some(decks);
        });
        app_state.deck_names = Some(decks);
    }
    let deck_names = app_state.deck_names.as_ref().unwrap();
    let mut selected = String::new();
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("Select a deck")
            .color(Color32::WHITE)
            .font(FontId::new(20.0, FontFamily::Proportional)));
        egui::ComboBox::from_label("Select a deck")
            .selected_text(app_state.selected_deck_name.as_ref().unwrap_or(&"".to_string()))
            .truncate()
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                ui.vertical_centered(|ui| {
                    for deck in deck_names {
                        ui.selectable_value(&mut selected, deck.to_string(), deck);
                    }
                });
            });
    });
    if selected != "" {
        app_state.selected_deck_name = Some(selected);
    }
    //println!("{:?}", selected);
}

pub fn display_new_words_count(
    app_state: &AppState,
    ui: &mut egui::Ui,
    new_words_count: usize,
    already_added_words_count: usize,
    all_words_in_kobo_count: usize,
    words_with_no_definitions: usize,

) {
    let already_added_words_count = already_added_words_count + app_state.processed_words.lock().unwrap().clone() as usize;
    let new_words_count = new_words_count - app_state.processed_words.lock().unwrap().clone() as usize;

    let new_words_count_layout = generate_layout_for_display_new_words_count("New words to add: ", new_words_count.to_string().as_str(), Color32::GREEN);
    let already_added_words_count_layout = generate_layout_for_display_new_words_count("Words already added: ", already_added_words_count.to_string().as_str(), Color32::LIGHT_BLUE);
    let all_words_in_kobo_count_layout = generate_layout_for_display_new_words_count("All words in Kobo: ", all_words_in_kobo_count.to_string().as_str(), Color32::RED);
    let words_with_no_definitions_layout = generate_layout_for_display_new_words_count("Words with no definitions: ", words_with_no_definitions.to_string().as_str(), Color32::LIGHT_RED);

    ui.vertical_centered(|ui| {
        ui.label(new_words_count_layout);
        ui.label(already_added_words_count_layout);
        ui.label(all_words_in_kobo_count_layout);
        ui.label(words_with_no_definitions_layout);
    });
}

pub fn display_start_button(app_state: &mut AppState, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        if ui.button(
            RichText::new("Start Sync")
                .color(Color32::LIGHT_BLUE)
                .font(FontId::new(20.0, FontFamily::Proportional))
        ).clicked() {
            if app_state.sync_started {
                return;
            }
            app_state.sync_started_at = Some(std::time::Instant::now());
            app_state.sync_started = true;
            let sync_progress = Arc::clone(&app_state.sync_progress);
            let words_to_add = app_state.words_to_add.as_ref().unwrap().clone();
            let anki_client = app_state.anki_client.clone();
            let connection_status = Arc::clone(&app_state.server_connection_status);
            let deck_name = app_state.selected_deck_name.as_ref().unwrap().clone();
            let processed_words = Arc::clone(&app_state.processed_words);
            let kobo_words = app_state.prepared_words_from_kobo.as_ref().unwrap();

            let new_words = app_state.words_to_add.as_ref().unwrap();
            let new_words_count = new_words.len();
            let already_added_words_count = kobo_words.len() - new_words_count;

            let words_with_no_definitions = Arc::clone(&app_state.words_with_no_definitions);

            app_state.async_rt.spawn(async move {
                AppState::sync_kobo_to_anki(
                    already_added_words_count as u32,
                    sync_progress,
                    words_to_add,
                    &anki_client,
                    &deck_name,
                    connection_status,
                    processed_words,
                    words_with_no_definitions,
                ).await;
            });

        }
    });
}

pub fn show_sync_started_message(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("Sync started!")
            .color(Color32::GOLD)
            .font(FontId::new(20.0, FontFamily::Proportional)));
    });
}

pub fn show_server_connection_error_message(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("Could not connect to the server!")
            .color(Color32::LIGHT_RED)
            .font(FontId::new(20.0, FontFamily::Proportional)));
    });
}

pub fn show_eta_message(app_state: &AppState, ui: &mut egui::Ui) {
    let sync_started_at = app_state.sync_started_at.unwrap();
    let elapsed = sync_started_at.elapsed();
    let eta = {
        let elapsed = elapsed.as_secs();

        let avg_time_per_unit = app_state.sync_progress.lock().unwrap().clone() as f64 / elapsed as f64;
        if app_state.sync_progress.lock().unwrap().clone() == 0.0 || app_state.sync_progress.lock().unwrap().clone() >= 100.0 {
            return;
        }
        //println!("{}", app_state.sync_progress.lock().unwrap().clone());
        let remaining = 100.0 - app_state.sync_progress.lock().unwrap().clone();
        let eta = remaining as f64 / avg_time_per_unit;
        let hours = eta / 3600.0;
        let minutes = (eta % 3600.0) / 60.0;
        let seconds = eta % 60.0;

        if hours > 0.0 {
            format!("{:.0}h {:.0}m {:.0}s", hours, minutes, seconds)
        } else if minutes > 0.0 {
            format!("{:.0}m {:.0}s", minutes, seconds)
        } else {
            format!("{:.0}s", seconds)
        }

    };
    let eta_layout = generate_layout_for_eta_message(&eta);
    ui.vertical_centered(|ui| {
        ui.label(eta_layout);
    });
}

pub fn show_progress_bar(app_state: &AppState, ui: &mut egui::Ui) {
    let progress = app_state.sync_progress.lock().unwrap();
    ui.add(egui::ProgressBar::new(*progress as f32 / 100.0).animate(true));
}

pub fn show_done_message(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("Sync completed!")
            .color(Color32::GREEN)
            .font(FontId::new(20.0, FontFamily::Proportional)));
    });
}

fn generate_layout_for_eta_message(
    eta: &str,
) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.append(
        "ETA: ",
        0.0,
        TextFormat {
            font_id: FontId::new(20.0, FontFamily::Proportional),
            color: Color32::LIGHT_BLUE,
            ..Default::default()
        },
    );
    job.append(
        eta,
        0.0,
        TextFormat {
            font_id: FontId::new(20.0, FontFamily::Proportional),
            color: Color32::LIGHT_YELLOW,
            italics: true,
            ..Default::default()
        },
    );

    job
}

fn generate_layout_for_display_new_words_count(
    string_left: &str,
    string_right: &str,
    right_color: Color32,
) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.append(
        string_left,
        0.0,
        TextFormat {
            font_id: FontId::new(20.0, FontFamily::Proportional),
            color: Color32::WHITE,
            ..Default::default()
        },
    );
    job.append(
        format!("{}", string_right).as_str(),
        0.0,
        TextFormat {
            font_id: FontId::new(22.0, FontFamily::Proportional),
            italics: true,
            color: right_color,
            ..Default::default()
        },
    );

    job
}
