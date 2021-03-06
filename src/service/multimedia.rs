use serde_json::Value;
use parser::Category;
use service::Service;
use service::Outcome;
use intent::Intent;

#[derive(PartialEq, Eq, Clone)]
pub struct Multimedia {
  pub categories: Vec<Category>,
}

impl Service for Multimedia {
  fn categories(&self) -> &Vec<Category> {
    &self.categories
  }

  // #[action(actions=[], subjects=[], others=[])]
  fn handle_action(&self, intent: &Intent) -> Outcome {
    Outcome::Success(String::from("song.play"), Value::Null)
  }

  fn can_handle(&self) -> bool {
    true
  }
}

impl Multimedia {
  pub fn new() -> Multimedia {
    Multimedia{
      categories: vec![Category::Multimedia],
    } 
  }
}