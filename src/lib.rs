#![allow(dead_code)]
#![feature(conservative_impl_trait)]
#[macro_use]
extern crate lazy_static;
extern crate postgres;
extern crate serde_json;

mod translator;
mod parser;
mod intent;
mod dispatcher;
mod context;
mod service;
mod database;
mod normalizer;

pub fn run(phrase: String) {
  let normalized_input = normalize_input(phrase);
  let translated_phrase = translator::translate(normalized_input);
  let translated_normalized_phrase = normalizer::normalize_phrase(translated_phrase);
  let parsed_words = parser::parse_words(translated_normalized_phrase);
  let intent = intent::to_intent(parsed_words);
  println!("Intent {:?}", intent);

  let outcome = dispatcher::dispatch_intent(intent);
  println!("Outcome {:?}", outcome);
}

fn normalize_input(phrase: String) -> String {
  phrase.as_str().to_lowercase()
}
