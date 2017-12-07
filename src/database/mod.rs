use postgres::{Connection, TlsMode};

pub fn get_dictionaries() -> Vec<String> {
  let conn = Connection::connect(
    "postgres://gabrielgatu@localhost:5432/mozzie",
    TlsMode::None,
  ).unwrap();

  conn
    .query("SELECT table_name FROM dictionary_table_names", &[])
    .unwrap()
    .into_iter()
    .map(|row| row.get(0))
    .collect()
}

pub fn translate_word(word: &str, dictionary: &str) -> Option<String> {
  let query = format!(
    "SELECT translation FROM {} WHERE word = '{}'",
    dictionary,
    word
  );

  let conn = Connection::connect(
    "postgres://gabrielgatu@localhost:5432/mozzie",
    TlsMode::None,
  ).unwrap();

  let rows = &conn.query(&query, &[]).unwrap();

  if !rows.is_empty() {
    let translation: String = rows.get(0).get(0);
    Some(translation)
  } else {
    None
  }
}

pub fn parse_word(word: &str) -> Option<(String, String, String)> {
  let conn = Connection::connect(
    "postgres://gabrielgatu@localhost:5432/mozzie",
    TlsMode::None,
  ).unwrap();

  let rows = &conn
    .query("SELECT * FROM words WHERE word = $1", &[&word])
    .unwrap();

  if !rows.is_empty() {
    let row = &rows.get(0);

    let resolved_word = row.get(1);
    let category = row.get(2);
    let kind = row.get(3);

    Some((resolved_word, category, kind))
  } else {
    None
  }
}

pub fn is_unnecessary_word(word: &str) -> bool {
  let conn = Connection::connect(
    "postgres://gabrielgatu@localhost:5432/mozzie",
    TlsMode::None,
  ).unwrap();

  let rows = &conn
    .query("SELECT * FROM unnecessary_words WHERE word = $1", &[&word])
    .unwrap();

  !rows.is_empty()
}
