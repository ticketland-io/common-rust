use super::interface::Logger;

struct ConsoleLogger;

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
