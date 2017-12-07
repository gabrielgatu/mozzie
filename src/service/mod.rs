mod multimedia;

pub use service::multimedia::Multimedia;

use serde_json;
use parser::Category;
use intent::Intent;

pub trait Service: Sync {
  fn categories(&self) -> &Vec<Category>;
  fn handle_action(&self, intent: &Intent) -> Outcome;
  fn can_handle(&self) -> bool; // TODO: Takes an intent
}

#[derive(Debug)]
pub enum Outcome {
  Success(String, serde_json::Value),
  Failure(Error),
}

#[derive(Debug)]
pub enum Error {
  ServiceNotFound,
  MissingArgument,
}