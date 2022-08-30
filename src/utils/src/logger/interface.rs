pub trait Logger {
  fn error(&self, message: &str);
  fn warn(&self, message: &str);
  fn info(&self, message: &str);
  fn debug(&self, message: &str);
}
