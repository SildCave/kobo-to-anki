use std::time::Duration;

use eframe::egui;

use crate::anki::AnkiClient;
use crate::anki::AnkiConnectionStatus;
use crate::cards::extract_words_from_anki_cards;
use crate::cards::prepare_words;
use crate::kobo::get_words_from_kobo_db;
use crate::AppState;
use crate::layout;

use crate::kobo::{find_and_validate_kobo_path, validate_kobo_path};

pub fn update_ui(
    app_state: &mut AppState,
    ctx: &egui::Context,
) {

    //let connection_status = AnkiClient::check_connection_non_blocking(app_state);
    if app_state.last_connection_attempt_time.elapsed().as_secs() > 2 {
        app_state.last_connection_attempt_time = std::time::Instant::now();
        let connection_status = AnkiClient::check_connection_non_blocking(app_state);
        if let Ok(status) = connection_status {
            app_state.anki_connection_status = status;
        }
    }

    let fresh_kobo_path = find_and_validate_kobo_path();
    if app_state.kobo_path.is_none() {
        app_state.kobo_path = fresh_kobo_path.clone();
    }
    if app_state.kobo_path.is_some() && fresh_kobo_path.is_none() {
        if !app_state.custom_path {
            app_state.kobo_path = None;
        }
    }
    egui::TopBottomPanel::top("Kobo To Anki Sync Tool").show(ctx, |ui| {
        layout::draw_header(ui);
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        layout::draw_kobo_connection_information_message(app_state, ui);

        if app_state.kobo_path.is_none() {
            layout::draw_kobo_connection_status_message_when_no_device_is_detected(app_state, ui);
        } else {
            layout::draw_kobo_connection_status_message_when_device_is_detected(app_state, ui);
            app_state.invalid_kobo_path = false;
        }

        if app_state.invalid_kobo_path {
            layout::notify_user_about_invalid_kobo_path(ui);
        }

        ui.add_space(10.0);
        layout::draw_horizontal_line(ui);
        ui.add_space(10.0);

        layout::display_anki_connection_status_message(app_state, ui);
        if app_state.anki_connection_status != crate::anki::AnkiConnectionStatus::Connected {
            layout::draw_anki_connection_guide(ui);
        } else if app_state.sync_started == false && !(app_state.prepared_words_from_anki.is_some() && app_state.prepared_words_from_kobo.is_some() && app_state.kobo_path.is_some()) {
            ui.add_space(10.0);
            layout::draw_horizontal_line(ui);
            layout::display_deck_selection_dropdown(app_state, ui);
        }

        ui.add_space(10.0);
        layout::draw_horizontal_line(ui);
        ui.add_space(10.0);

        if app_state.prepared_words_from_kobo.is_some() && app_state.prepared_words_from_anki.is_some() {
            let kobo_words = app_state.prepared_words_from_kobo.as_ref().unwrap();
            let anki_words = app_state.prepared_words_from_anki.as_ref().unwrap();
            if app_state.words_to_add.is_none() {
                let mut new_words = vec![];
                for word in kobo_words {
                    if !anki_words.contains(word) {
                        new_words.push(word.clone());
                    }
                }
                app_state.words_to_add = Some(new_words);
            }
            let new_words = app_state.words_to_add.as_ref().unwrap();

            //println!("new words count: {:?}", new_words);
            let new_words_count = new_words.len();

            let already_added_words = kobo_words.len() - new_words_count;
            let total_words_in_kobo_count = kobo_words.len();

            let words_with_no_definitions = app_state.words_with_no_definitions.lock().unwrap().clone() as usize;

            layout::display_new_words_count(
                &app_state,
                ui,
                new_words_count,
                already_added_words,
                total_words_in_kobo_count,
                words_with_no_definitions,
            );
            ui.add_space(10.0);
            layout::draw_horizontal_line(ui);
            ui.add_space(10.0);

            if app_state.sync_started == false {
                layout::display_start_button(app_state, ui);
            }

            if app_state.sync_started && app_state.server_connection_status.lock().unwrap().clone() == true {
                if app_state.sync_progress.lock().unwrap().clone() >= 100.0 {
                    layout::show_done_message(ui);
                } else {
                    layout::show_sync_started_message(ui);
                }
                layout::show_eta_message(app_state, ui);
            }

            if app_state.sync_started && app_state.server_connection_status.lock().unwrap().clone() == true {
                ui.add_space(10.0);
                layout::draw_horizontal_line(ui);
                ui.add_space(10.0);

                layout::show_progress_bar(app_state, ui);
            }

            // if app_state.sync_progress.lock().unwrap().clone() >= 100 {
            //     ui.add_space(10.0);
            //     layout::draw_horizontal_line(ui);
            //     ui.add_space(10.0);

            //     layout::show_done_message(&app_state, ui);
            // }
        }
        if app_state.server_connection_status.lock().unwrap().clone() == false {
            layout::show_server_connection_error_message(ui);
        }
    });

    app_state.file_dialog.update(ctx);

    // Check if the user selected a file.
    if let Some(path) = app_state.file_dialog.take_selected() {
        if validate_kobo_path(&path) {
            app_state.kobo_path = Some(path);
            app_state.invalid_kobo_path = false;
            app_state.custom_path = true;

        } else {
            app_state.kobo_path = None;
            app_state.invalid_kobo_path = true;
        }
    };

    if app_state.anki_connection_status == AnkiConnectionStatus::Connected && app_state.raw_cards_from_anki.is_none() && app_state.selected_deck_name.is_some() {
        let async_rt = &app_state.async_rt;
        async_rt.block_on(async {
            let cards = app_state.anki_client.get_cards_from_anki_deck(app_state.selected_deck_name.as_ref().unwrap()).await;
            let formated_cards = extract_words_from_anki_cards(&cards);


            //println!("len: {}", formated_cards.len());
            app_state.prepared_words_from_anki = Some(formated_cards);
            app_state.raw_cards_from_anki = Some(cards);
        });
    }

    if app_state.kobo_path.is_some() && app_state.invalid_kobo_path == false && app_state.prepared_words_from_kobo.is_none() {
        let words = get_words_from_kobo_db(
            &app_state.kobo_path.as_ref().unwrap()
        ).unwrap();
        app_state.prepared_words_from_kobo = Some(
            prepare_words(words)
        );
        //println!("{:?}", app_state.prepared_words_from_kobo);
    }

    // if app_state.prepared_words_from_kobo.is_some() && app_state.prepared_words_from_anki.is_some() {
    //     let kobo_words = app_state.prepared_words_from_kobo.as_ref().unwrap();
    //     let anki_words = app_state.prepared_words_from_anki.as_ref().unwrap();
    //     let mut new_words = vec![];
    //     for word in kobo_words {
    //         if !anki_words.contains(word) {
    //             new_words.push(word.clone());
    //         }
    //     }
    //     println!("kobo words count: {}", kobo_words.len());
    //     println!("new words count: {}", new_words.len());
    //     println!("old words count: {}", kobo_words.len() - new_words.len());
    // }

    ctx.request_repaint_after(Duration::from_millis(300));
}