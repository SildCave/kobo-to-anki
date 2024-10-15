use std::sync::Arc;

use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse
};
use log::info;
use serde::{
    Serialize, Deserialize
};

use crate::database::DatabaseClient;

use super::cambrinary_bindings::{fetch_word_from_cambrinary, CambrinarySessionTracker};

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    word: String,
}

#[derive(Clone)]
pub struct GetWordState {
    pub db_client: DatabaseClient,
    pub cambrinary_session_tracker: Arc<CambrinarySessionTracker>,
}


#[axum::debug_handler]
pub async fn get_word(
    Path(requested_word): Path<Params>,
    State(get_word_state): State<GetWordState>,
) -> impl IntoResponse {
    let db_client = &get_word_state.db_client;
    let cambrinary_session_tracker = &get_word_state.cambrinary_session_tracker;

    let requested_word = requested_word.word;

    loop {
        // Look for the word in the database
        let word_from_db = db_client.try_to_get_word(&requested_word).await;
        println!("{:?}", word_from_db);
        if word_from_db.is_err() {
            tracing::error!("{:?}", word_from_db.err().unwrap());
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string());
        }
        let word_from_db = word_from_db.unwrap();
        if word_from_db.is_some() {
            let word = word_from_db.unwrap();
            let res = (&db_client).update_word_metadata(&word.word).await;
            match res {
                Ok(_) => info!("Word {} metadata updated", word.word),
                Err(e) => tracing::warn!("{:?}", e),
            }
            let word = serde_json::to_string(&word).unwrap();
            return (StatusCode::OK, word);
        }
        info!("Word {} not found in the database, attempting to acquire a session", requested_word);

        // If the word is not found in the database, try to fetch it from cambridge dictionary
        let can_start_session = cambrinary_session_tracker.can_start_session().await;
        if can_start_session {
            break;
        }
        cambrinary_session_tracker.wait_for_session().await;
    }
    // Start a session and fetch the word
    cambrinary_session_tracker.start_session().await;
    let word = fetch_word_from_cambrinary(&requested_word).await;
    cambrinary_session_tracker.end_session().await;

    if word.is_err() {
        tracing::error!("{:?}", word.err().unwrap());
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string());
    }
    let word = word.unwrap();
    if word.is_none() {
        tracing::info!("Word {} not found", requested_word);
        return (StatusCode::NOT_FOUND, "Word not found".to_string());
    }
    let word = word.unwrap();
    db_client.insert_word(word.clone()).await.unwrap();
    info!("Word {} inserted into the database", requested_word);

    let word = serde_json::to_string(&word).unwrap();
    (StatusCode::OK, word)
}

