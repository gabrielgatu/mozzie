mod multimedia;

pub use service::multimedia::Multimedia;

use parser::Category;

pub trait Service: Sync {
  fn categories(&self) -> &Vec<Category>;
  fn handle_action(&self);
  fn can_handle(&self) -> bool; // TODO: Takes an intent
}