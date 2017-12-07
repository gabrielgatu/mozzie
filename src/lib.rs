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

pub fn run(phrase: String) {
  let normalized_phrase = normalize_phrase(phrase);
  let translated_phrase = translator::translate(normalized_phrase);
  let parsed_words = parser::parse_words(translated_phrase);
  let intent = intent::to_intent(parsed_words);
  println!("Intent {:?}", intent);

  let outcome = dispatcher::dispatch_intent(intent);
  println!("Outcome {:?}", outcome);
}

fn normalize_phrase(phrase: String) -> String {
  phrase.as_str().to_lowercase()
}