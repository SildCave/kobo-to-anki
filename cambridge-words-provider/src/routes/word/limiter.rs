use std::{sync::Arc, time::Duration};

use tower_governor::{governor::{GovernorConfig, GovernorConfigBuilder}, key_extractor::{GlobalKeyExtractor, PeerIpKeyExtractor}};


pub fn create_limiter_for_get_words_endpoint(
    per_second: u64,
    burst_size: u32,
) -> Arc<GovernorConfig<PeerIpKeyExtractor, governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>>> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(per_second)
            .burst_size(burst_size)
            .finish()
            .unwrap(),
    );
    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    // a separate background task to clean up
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            tracing::info!("rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    governor_conf
}

pub fn create_global_limiter_for_get_words_endpoint(
    per_second: u64,
    burst_size: u32,
) -> Arc<GovernorConfig<GlobalKeyExtractor, governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>>> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(per_second)
            .burst_size(burst_size)
            .key_extractor(GlobalKeyExtractor)
            .finish()
            .unwrap(),
    );
    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    // a separate background task to clean up
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            tracing::info!("rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    governor_conf
}