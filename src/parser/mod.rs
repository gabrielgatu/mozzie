mod word;

pub use parser::word::Word;

/// Identifies the category the word belongs to.
/// Most words don't belong to any category (it cannot be inferred),
/// so the default `Unknown` is used.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Category {
  Multimedia,
  Fitness,
  Agenda,
  Unknown,
}

impl Category {
  fn from_string(val: &str) -> Category {
    match val {
      "multimedia" => Category::Multimedia,
      "fitness" => Category::Fitness,
      "agenda" => Category::Agenda,
      _ => Category::Unknown,
    }
  }
}

/// Identifies the type of the word.
#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
  Action,
  Subject,
  Platform,
  Service,
  Number,
  Date,
  Unknown,
  Other(String),
}

impl Kind {
  fn from_string(val: &str) -> Kind {
    match val {
      "action" => Kind::Action,
      "subject" => Kind::Subject,
      "platform" => Kind::Platform,
      "service" => Kind::Service,
      "number" => Kind::Number,
      "date" => Kind::Date,
      "unknown" => Kind::Unknown,
      other => Kind::Other(other.to_string()),
    }
  }
}

/// Wrapper containing the parsed value of a word.
/// I.e. If the word is a number, it parses it into f32 and stores inside a Num(f32).
#[derive(Clone, Debug)]
pub enum Value {
  Str(String),
  Num(f32),
}

/// Parses a vector of english words, and for each one
/// it identifies the category it belongs to, its type (kind),
/// and parses the word into its true type (aka: number types become f32).
pub fn parse_words(words: Vec<String>) -> Vec<Word> {
  words.into_iter().map(|word| Word::parse(word)).collect()
}
