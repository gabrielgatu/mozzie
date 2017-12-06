pub struct HistoryStack<T> {
  stack: Vec<T>,
}

impl<T> HistoryStack<T> {
  pub fn new() -> HistoryStack<T> {
    HistoryStack { stack: Vec::new() }
  }

  pub fn all(&self) -> &Vec<T>
  {
    &self.stack
  }

  pub fn push(&mut self, val: T)
  where
    T: Eq,
  {
    if let Some(index) = self.index_of(&val) {
      self.stack.remove(index);
    }
    self.stack.insert(0, val);
  }

  pub fn remove(&mut self, val: &T) -> Option<T>
  where
    T: Eq,
  {
    if let Some(index) = self.index_of(&val) {
      Some(self.stack.remove(index))
    } else {
      None
    }
  }

  pub fn index_of(&self, val: &T) -> Option<usize>
  where
    T: Eq,
  {
    (&self.stack).into_iter().position(|item| item == val)
  }
}
