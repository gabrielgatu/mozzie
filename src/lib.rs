#![allow(dead_code)]
#![feature(conservative_impl_trait)]
#[macro_use]

extern crate lazy_static;
extern crate postgres;

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
  println!("{:?}", intent);

  dispatcher::dispatch_intent(intent);
}

fn normalize_phrase(phrase: String) -> String {
  phrase.as_str().to_lowercase()
}