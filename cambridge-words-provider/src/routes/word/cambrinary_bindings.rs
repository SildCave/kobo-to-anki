use std::time::Duration;

use rand::Rng;
use serde::{
    Deserialize, Serialize
};
use tokio::{process::Command, sync::RwLock};

use anyhow::Result;
use tracing::{debug, info};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Word {
    pub word: String,
    pub meanings_with_examples: Vec<MeaningWithExamples>,
    pub has_meaning: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeaningWithExamples {
    pub meaning: String,
    pub examples: Vec<String>,
}

impl Word {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Default for Word {
    fn default() -> Self {
        Self {
            word: "".to_string(),
            meanings_with_examples: vec![],
            has_meaning: false,
        }
    }
    
}

pub struct CambrinarySessionTracker {
    pub sessions: RwLock<u32>,
    pub max_sessions: u32,
    pub session_acquire_cooldown: u64,
    pub session_acquire_cooldown_jitter: u64,
    pub session_clean_interval_s: u64,
    last_clean_time: RwLock<std::time::Instant>,

}

impl CambrinarySessionTracker {
    pub fn new(
        max_sessions: u32,
        session_acquire_cooldown: u64,
        session_acquire_cooldown_jitter: u64,
    ) -> Self {
        Self {
            sessions: RwLock::new(0),
            max_sessions,
            session_acquire_cooldown,
            session_acquire_cooldown_jitter,
            session_clean_interval_s: 10,
            last_clean_time: RwLock::new(std::time::Instant::now()),
        }
    }

    async fn clean_sessions(&self) {
        let mut sessions = self.sessions.write().await;
        *sessions = 0;
        let mut last_clean_time = self.last_clean_time.write().await;
        *last_clean_time = std::time::Instant::now();
    }

    async fn can_clean_sessions(&self) -> bool {
        let last_clean_time = self.last_clean_time.read().await;
        let now = std::time::Instant::now();
        let duration = now.duration_since(*last_clean_time);
        duration.as_secs() >= self.session_clean_interval_s
    }

    pub async fn can_start_session(&self) -> bool {
        let sessions = self.get_sessions().await;
        sessions < self.max_sessions
    }

    pub async fn wait_for_session(&self) {
        info!("Current sessions: {}", self.get_sessions().await);
        if self.can_clean_sessions().await {
            self.clean_sessions().await;
            info!("Sessions cleaned");
        }
        let jitter = {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..self.session_acquire_cooldown_jitter)
        };
        tokio::time::sleep(
            Duration::from_millis(self.session_acquire_cooldown)
                + Duration::from_millis(
                    jitter,
                ),
        ).await;
    }

    pub async fn start_session(&self) {
        self.increment_session().await;
    }

    pub async fn end_session(&self) {
        self.decrement_session().await;
    }

    async fn increment_session(&self) {
        let mut sessions = self.sessions.write().await;
        *sessions += 1;
    }

    async fn decrement_session(&self) {
        let mut sessions = self.sessions.write().await;
        *sessions -= 1;
    }

    async fn get_sessions(&self) -> u32 {
        let sessions = self.sessions.read().await;
        *sessions
    }
}


pub async fn fetch_word_from_cambrinary(word: &str) -> Result<Option<Word>> {
    let output = Command::new("cambrinary")
        .arg("-w")
        .arg(word)
        .output();
    let output = output.await?;
    if output.status.code() != Some(0) {
        let status = output.status;
        let error = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        let error_message = format!(
            "status: {}\nerror: {}\nstdout: {}",
            status, error, stdout
        );

        return Err(anyhow::anyhow!(error_message));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str.contains("No result for ") {
        return Ok(None);
    }

    let meanings_with_examples = output_str.split("0;36;49m* ").collect::<Vec<&str>>();
    let mut word = Word {
        word: word.to_string(),
        meanings_with_examples: vec![],
        has_meaning: true,
    };
    for meaning_with_example in meanings_with_examples {
        if meaning_with_example.contains("[0;34;49m") {
            continue;
        }
        let meaning = meaning_with_example.split(": [0m").collect::<Vec<&str>>();
        if meaning.len() == 0 {
            // no meaning
            continue;
        }

        let meaning = meaning[0].split("\u{1b}").collect::<Vec<&str>>()[0];
        let meaning = sanitize_str(meaning);
        let examples = meaning_with_example.split("  - ").collect::<Vec<&str>>();
        let mut meaning_with_examples = MeaningWithExamples {
            meaning: meaning.to_string(),
            examples: vec![],
        };
        if examples.len() == 1 {
            // no examples
            word.meanings_with_examples.push(meaning_with_examples);
            continue;
        }
        let examples = &examples[1..];
        for example in examples {
            let example = sanitize_str(&example);
            let example: &str = &example.split("[0m").collect::<Vec<&str>>()[0];
            meaning_with_examples.examples.push(example.to_string());
        }
        if meaning_with_examples.examples.len() == 0 && meaning_with_examples.meaning.len() == 0 {
            continue;
        }
        word.meanings_with_examples.push(meaning_with_examples);
    }


    Ok(Some(word))
}

fn sanitize_str(word: &str) -> String {
    word.replace("\n", "").replace("\r", "").trim().to_string()
}