mod hello_world;
mod word;

use std::sync::Arc;

use axum::{
    routing::{
        get,
    }, Router
};

use tower_governor::GovernorLayer;
use word::{create_global_limiter_for_get_words_endpoint, create_limiter_for_get_words_endpoint, get_word, CambrinarySessionTracker};


pub use word::{
    Word,
    MeaningWithExamples
};

use crate::{configuration::{CambrinarySessionTrackerConfig, RateLimiterConfig}, database::DatabaseClient};
pub async fn configure_routes(
    db_client: DatabaseClient,
    rate_limiter_config: RateLimiterConfig,
    cambrinary_session_tracker_config: CambrinarySessionTrackerConfig
) -> Router {
    let get_word_limiter_config = create_limiter_for_get_words_endpoint(
        rate_limiter_config.max_per_second,
        rate_limiter_config.burst_size
    );
    let get_word_global_limiter_config = create_global_limiter_for_get_words_endpoint(
        rate_limiter_config.max_per_second_global,
        rate_limiter_config.burst_size_global
    );

    let cambrinary_session_tracker = Arc::new(CambrinarySessionTracker::new(
        cambrinary_session_tracker_config.max_sessions,
        cambrinary_session_tracker_config.session_acquire_cooldown_ms,
        cambrinary_session_tracker_config.session_acquire_cooldown_jitter_ms
    ));

    let get_word_state = word::GetWordState {
        db_client,
        cambrinary_session_tracker: cambrinary_session_tracker.clone(),
    };
    
    Router::new()
        .route("/get_word/:word", get(get_word))
        .with_state(get_word_state)
        .layer(
            GovernorLayer {
                config: get_word_global_limiter_config,
            }
        )
        .layer(
            GovernorLayer {
                config: get_word_limiter_config,
            }
        )
        .route("/health", get(|| async { "OK" }))
}



// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM' \
//     http://localhost:3000/protected