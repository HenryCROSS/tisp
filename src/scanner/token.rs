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
  Symbol(String),
  String(String),
  Character(char),
  Keyword(String),
  Float32(f32),
  Int32(i32),
  Bool(bool),

  // Keywords.
  And,
  Class,
  Else,
  Func,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  Var,
  While,
  Quote,
  Macro,


  // End of file.
  EOF,
} 

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub token_type: TokenType,
  pub line: u32,
  pub column: u32,
}
