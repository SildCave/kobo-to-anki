use anki_bridge::prelude::CardsInfoResponse;



pub fn prepare_words(words: Vec<String>) -> Vec<String> {
    let mut prepared_words = Vec::new();
    
    assert!(!','.is_alphabetic());
    assert!('a'.is_alphabetic());

    for word in words {
        let mut prepared_word = word.to_lowercase();
        prepared_word.retain(|c| (c.is_alphabetic() || c == ' '));
        prepared_words.push(prepared_word.trim().to_string());
    }
    prepared_words
}

pub fn extract_words_from_anki_cards(cards: &Vec<CardsInfoResponse>) -> Vec<String> {
    let cards = cards.clone();
    let mut words = Vec::new();
    for card in cards {
        let mut word = card.question;
        word = word.split("\">").last().unwrap().to_string();
        word = word.split("</span></center></center>").nth(0).unwrap().to_string();
        word = word.trim().to_string();
        words.push(word);
    }

    words = prepare_words(words);

    words
}