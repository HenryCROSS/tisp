#[derive(Debug, Clone)]
pub struct ParseError {
  pub message: String,
  pub line: u32,
  pub column: u32,
}

impl ParseError {
  pub fn new(message: &str, line: u32, column: u32) -> Self {
    ParseError {
      message: message.to_string(),
      line,
      column,
    }
  }
}

pub type ParseResult<T> = Result<T, ParseError>;
