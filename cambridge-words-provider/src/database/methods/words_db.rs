use super::super::DatabaseClient;

use crate::routes::{MeaningWithExamples, Word};

impl DatabaseClient {
    pub async fn insert_word(&self, word: Word) -> Result<(), sqlx::Error> {
        let meanings_with_examples = serde_json::to_string(&word.meanings_with_examples).unwrap();
        let res = sqlx::query!(
            r#"
            INSERT INTO words (word, meanings_with_examples, created_at, last_acsess_at, acsess_count, has_meaning)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            word.word,
            meanings_with_examples,
            chrono::Utc::now().timestamp(),
            chrono::Utc::now().timestamp(),
            0,
            word.has_meaning

        );
        res.execute(&self.postgres_con).await?;
        
        Ok(())
    }

    pub async fn update_word_metadata(&self, word: &str) -> Result<(), sqlx::Error> {
        let query = sqlx::query!(
            r#"
            UPDATE words SET last_acsess_at = $1, acsess_count = acsess_count + 1 WHERE word = $2
            "#,
            chrono::Utc::now().timestamp(),
            word
        );
        query.execute(&self.postgres_con).await?;

        Ok(())
    }

    pub async fn try_to_get_word(&self, word: &str) -> Result<Option<Word>, sqlx::Error> {
        let word = sqlx::query!(
            r#"
            SELECT * FROM words WHERE word = $1
            "#,
            word
        ).fetch_optional(&self.postgres_con).await?;

        if word.is_none() {
            return Ok(None);
        }

        let word = word.unwrap();
        let meanings_with_examples = word.meanings_with_examples;
        let meanings_with_examples: Vec<MeaningWithExamples> = serde_json::from_str(&meanings_with_examples).unwrap();
        let word = Word {
            word: word.word,
            meanings_with_examples,
            has_meaning: word.has_meaning
        };
        println!("{:?}", word.word);

        Ok(Some(word))
    }

}