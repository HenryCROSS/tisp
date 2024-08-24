#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  // Single-character tokens.
  LeftParen,  // (
  RightParen, // )
  LeftBrace,  // {
  RightBrace, // }
  Comma,      // ,
  Dot,        // .
  Minus,      // -
  Plus,       // +
  Semicolon,  // ;
  Slash,      // /
  Star,       // *

  // One or two character tokens.
  Bang,         // !
  BangEqual,    // !=
  Equal,        // =
  EqualEqual,   // ==
  Greater,      // >
  GreaterEqual, // >=
  Less,         // <
  LessEqual,    // <=

  // Literals.
  Identifier(String),
  String(String),
  Double(f32),
  Int(i32),

  // Keywords.
  And,
  Class,
  Else,
  False,
  Func,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,

  // End of file.
  EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub token_type: TokenType,
  pub line: u32,
  pub column: u32,
}
