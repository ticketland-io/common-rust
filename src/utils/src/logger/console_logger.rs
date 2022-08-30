use std::sync::Arc;
use lazy_static::lazy_static;
use super::interface::Logger;

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
  fn error(&self, message: &str) {
    println!("[Error]: {}", message);
  }

  fn warn(&self, message: &str) {
    println!("[Warning]: {}", message);
  }

  fn info(&self, message: &str) {
    println!("[Info]: {}", message);
  }

  fn debug(&self, message: &str) {
    println!("[Debug]: {}", message);
  }
}

lazy_static! {
  pub static ref LOGGER: Arc<ConsoleLogger> = Arc::new(
    ConsoleLogger {}
  );
}
