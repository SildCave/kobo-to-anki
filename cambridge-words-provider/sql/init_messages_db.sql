-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS words (
        id SERIAL PRIMARY KEY,
        word TEXT NOT NULL,
        meanings_with_examples TEXT NOT NULL,
        created_at BIGINT NOT NULL,
        acsess_count INT NOT NULL,
        last_acsess_at BIGINT NOT NULL,
        has_meaning BOOLEAN NOT NULL
    );