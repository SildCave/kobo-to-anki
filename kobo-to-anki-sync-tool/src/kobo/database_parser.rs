use rusqlite;


pub fn get_words_from_kobo_db(
    kobo_path: &std::path::PathBuf,
) -> Result<Vec<String>, rusqlite::Error> {
    let kobo_reader_sqlite = kobo_path.join(".kobo/KoboReader.sqlite");
    let conn = rusqlite::Connection::open(kobo_reader_sqlite)?;
    let mut stmt = conn.prepare("SELECT * FROM WordList")?;
    let words = stmt
        .query_map([], |row| {
            Ok(row.get(0)?)
        })?
        .map(|word| word.unwrap())
        .collect::<Vec<String>>();
    Ok(words)
}