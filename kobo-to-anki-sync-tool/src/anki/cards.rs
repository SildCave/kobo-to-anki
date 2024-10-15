use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use super::AnkiClient;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Word {
    pub word: String,
    pub meanings_with_examples: Vec<MeaningWithExamples>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeaningWithExamples {
    pub meaning: String,
    pub examples: Vec<String>,
}

impl<'a> AnkiClient<'a> {
    pub async fn add_card_with_fromating(&self, word: &str, deck_name: &str, connection_status: Arc<Mutex<bool>>) {
        let front = format!("<center><span style=\"font-size:3em;color:red\">{}</span></center>", word);
        let mut back = String::new();
        let word = self.get_word_from_api(word, connection_status).await.unwrap();
        for meaning_with_examples in word.meanings_with_examples {
            back.push_str(&format!("<span style=\"font-size:1.5em;color:cyan\">{}</span><br />", meaning_with_examples.meaning));
            for example in meaning_with_examples.examples {
                back.push_str(&format!("<span style=\"font-size:0.7em;color:white\">{}</span><br />", example));
            }
        }
        self.add_card_to_deck(
            deck_name,
            format!("<center>{}</center>", front).as_str(),
            format!("<center>{}</center>", back).as_str()
        ).await.unwrap();
    }

    pub async fn get_word_from_api(&self, word: &str, connection_status: Arc<Mutex<bool>>) -> Option<Word> {
        let cambridge_words_provider_url = &String::from_utf8_lossy(include_bytes!("../../server_addr")).to_string();
        let url = format!("{}/get_word/{}", cambridge_words_provider_url, word);
        let response = reqwest::get(url).await;
        let word = match response {
            Ok(response) => {
                if response.status() == 404 {
                    println!("Word {} not found", word);
                    return None;
                }
                if response.status() == 500 {
                    println!("Internal server error");
                    return None;
                }
                if response.status() == 429 {
                    tokio::time::sleep(std::time::Duration::from_secs(25)).await;
                    let url = format!("{}/get_word/{}", cambridge_words_provider_url, word);
                    let response = reqwest::get(url).await;
                    if response.is_err() {
                        let mut status = connection_status.lock().unwrap();
                        *status = false;
                        println!("{:?}", response.err().unwrap());
                        return None;
                    }
                    let response = response.unwrap();
                    let word = response.json::<Word>().await.unwrap();
                    return Some(word);

                }
                let word = response.json::<Word>().await.unwrap();
                Some(word)
            }
            Err(e) => {
                let mut status = connection_status.lock().unwrap();
                *status = false;
                println!("{:?}", e);
                None
            }
        };
        word
    }
}