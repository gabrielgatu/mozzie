use database;

enum Word {
  Val(String),
  Translation(String),
}

impl Word {
  fn get(self) -> String {
    match self {
      Word::Translation(val) => val,
      Word::Val(val) => val,
    }
  }

  fn has_translation(&self) -> bool {
    match *self {
      Word::Translation(_) => true,
      Word::Val(_) => false,
    }
  }
}

/// Takes a phrase and splits it by whitespace.
/// For each word found, it tries to translate it. If no translation
/// is found, then the original one is kept.
/// 
/// Inside it has a mechanism for auto-selecting the language,
/// based on the number of matches in each language.
pub fn translate(phrase: String) -> Vec<String> {
  let words: Vec<&str> = phrase.split_whitespace().collect();

  let translations: Vec<Vec<Word>> = database::get_dictionaries()
    .into_iter()
    .map(|dictionary| {
      (&words)
        .into_iter()
        .map(|word| match database::translate_word(word, &dictionary) {
          Some(translation) => Word::Translation(translation),
          None => Word::Val(word.to_string()),
        })
        .collect()
    })
    .collect();

  let translation_with_most_matches: Vec<Word> = translations
    .into_iter()
    .max_by(|t1, t2| {
      let c1 = t1.into_iter().filter(|word| word.has_translation()).count();
      let c2 = t2.into_iter().filter(|word| word.has_translation()).count();
      c1.cmp(&c2)
    })
    .unwrap();

  let translated_phrase: Vec<String> = translation_with_most_matches
    .into_iter()
    .map(|word| word.get())
    .collect();

  translated_phrase
}
