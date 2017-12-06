use database;
use parser::{Value, Category, Kind};

#[derive(Clone, Debug)]
pub struct Word {
  pub word: String,
  pub value: Value,
  pub category: Category,
  pub kind: Kind,
}

impl Word {
  pub fn new(word: String, value: Value, category: Category, kind: Kind) -> Word {
    Word {
      word: word,
      value: value,
      category: category,
      kind: kind,
    }
  }

  pub fn parse(word: String) -> Word {
    if let Some(word) = Word::find_translation(&word) {
      return word;
    }

    if let Ok(val) = word.as_str().parse::<f32>() {
      return Word::new(word, Value::Num(val), Category::Unknown, Kind::Unknown);
    }

    Word::new(
      word.clone(),
      Value::Str(word),
      Category::Unknown,
      Kind::Unknown,
    )
  }

  pub fn find_translation(word: &str) -> Option<Word> {
    let (resolved_word, category, kind) = database::parse_word(word)?;

    Some(Word::new(
      word.to_string(),
      Value::Str(resolved_word),
      Category::from_string(&category),
      Kind::from_string(&kind),
    ))
  }
}