use std::sync::{Arc, Mutex};

use crate::{anki, AppState};


impl AppState {
    pub async fn sync_kobo_to_anki(
        already_processed_words: u32,
        progress: Arc<Mutex<f32>>,
        words_to_add: Vec<String>,
        anki_client: &anki::AnkiClient<'static>,
        deck_name: &str,
        connection_status: Arc<Mutex<bool>>,
        processed_words: Arc<Mutex<u32>>,
        words_with_no_definitions: Arc<Mutex<u32>>,
    ) {
        let word_count = words_to_add.len() as u32;
        let mut current_word_num = 0;
        for word in words_to_add {
            let current_word = word.clone();
            current_word_num += 1;
            *processed_words.lock().unwrap() = current_word_num;
            println!("Current word num: {}, word: {}", current_word_num, current_word);
            println!("already_processed_words: {}, word_count: {}, current_word_num: {}", already_processed_words, word_count, current_word_num);

            if connection_status.lock().unwrap().clone() == false {
                return;
            }
            let connection_status = connection_status.clone();
            let word = anki_client.get_word_from_api(&word, connection_status.clone()).await;
            if word.is_none() {
                println!("No definition found for the word: {:?}", word);
                let mut progress = progress.lock().unwrap();
                *progress = {
                    100.0 * current_word_num as f32 / (word_count) as f32
                };
                let mut words_with_no_definitions = words_with_no_definitions.lock().unwrap();
                if connection_status.lock().unwrap().clone() == false {
                    return;
                }
                *words_with_no_definitions += 1;
                continue;
            }

            anki_client.add_card_with_fromating(
                &word.unwrap().word,
                deck_name,
                connection_status
            ).await;
            //println!("{:?}", word);
            let mut progress = progress.lock().unwrap();
            
            *progress = {
                100.0 * current_word_num as f32 / (word_count) as f32
            };
        }

    }
}
