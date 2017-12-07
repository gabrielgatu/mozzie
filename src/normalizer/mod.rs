use database;

pub fn normalize_phrase(words: Vec<String>) -> Vec<String> {
  words
    .into_iter()
    .filter(|word| !is_unnecessary_word(word))
    .collect()
}

fn is_unnecessary_word(word: &str) -> bool {
  database::is_unnecessary_word(word)
}
