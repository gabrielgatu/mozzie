use parser::Word;
use parser::Category;
use parser::Kind;
use std::collections::HashMap;

/// An Intent contains all the informations about
/// the action the user wants to perform, categorized by
/// their type. 
#[allow(dead_code)]
#[derive(Debug)]
pub struct Intent {
  pub category: Category,
  pub actions: Vec<Word>,
  pub subjects: Vec<Word>,
  pub values: Vec<Word>,
  pub times: Vec<Word>,
  pub services: Vec<Word>,
  pub platforms: Vec<Word>,
  pub others: Vec<Word>,
}

pub fn to_intent(words: Vec<Word>) -> Intent {
  let actions = filter_by_type(&words, Kind::Action);
  let subjects = filter_by_type(&words, Kind::Subject);
  let values = filter_by_type(&words, Kind::Number);
  let times = filter_by_type(&words, Kind::Date);
  let services = filter_by_type(&words, Kind::Service);
  let platforms = filter_by_type(&words, Kind::Platform);
  let others = filter_by_type(&words, Kind::Unknown);
  let category = get_category(&subjects, &actions, &others);

  Intent {
    category: category,
    actions: actions,
    subjects: subjects,
    values: values,
    times: times,
    services: services,
    platforms: platforms,
    others: others,
  }
}

fn filter_by_type(words: &Vec<Word>, kind: Kind) -> Vec<Word> {
  words
    .into_iter()
    .filter(|word| word.kind == kind)
    .map(|word| word.clone())
    .collect::<Vec<_>>()
}

fn get_category(subjects: &Vec<Word>, actions: &Vec<Word>, others: &Vec<Word>) -> Category {
  if subjects.len() > 0 {
    return get_category_with_max_occurences(subjects);
  } else if actions.len() > 0 {
    return get_category_with_max_occurences(actions);
  } else {
    get_category_with_max_occurences(others)
  }
}

fn get_category_with_max_occurences(words: &Vec<Word>) -> Category {
  let res = words.iter().fold(HashMap::new(), |mut acc, word| {
    {
      let counter = acc.entry(&word.category).or_insert(0);
      *counter += 1;
    }
    acc
  });

  if let Some(res) = res.iter().max_by(|a, b| a.1.cmp(b.1)) {
    return (*res.0).clone();
  } else {
    return Category::Unknown;
  }
}
