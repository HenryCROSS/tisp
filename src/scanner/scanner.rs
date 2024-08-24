use std::env;
use std::fs;

use super::token::Token;
use super::token::TokenType;

fn read_file_scan(file_name: String) -> Result<Vec<Token>, Vec<String>> {
  fs::read_to_string(file_name)
    .map_err(|e| vec![e.to_string()])
    .and_then(read_str_scan)
}

fn read_str_scan(text: String) -> Result<Vec<Token>, Vec<String>> {
  let mut tokens = Vec::new();
  let mut errors = Vec::new();
  let mut chars = text.chars().peekable();
  let mut line = 1;
  let mut column = 0;

  while let Some(c) = chars.next() {
    column += 1;
    let token_type = match c {
      '(' => TokenType::LeftParen,
      ')' => TokenType::RightParen,
      '{' => TokenType::LeftBrace,
      '}' => TokenType::RightBrace,
      ',' => TokenType::Comma,
      '.' => TokenType::Dot,
      '-' => TokenType::Minus,
      '+' => TokenType::Plus,
      ';' => TokenType::Semicolon,
      '/' => TokenType::Slash,
      '*' => TokenType::Star,
      '!' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          TokenType::BangEqual
        } else {
          TokenType::Bang
        }
      }
      '=' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          TokenType::EqualEqual
        } else {
          TokenType::Equal
        }
      }
      '>' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          TokenType::GreaterEqual
        } else {
          TokenType::Greater
        }
      }
      '<' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          TokenType::LessEqual
        } else {
          TokenType::Less
        }
      }
      ' ' | '\r' | '\t' => continue,
      '\n' => {
        line += 1;
        column = 0;
        continue;
      }
      _ => {
        if c.is_digit(10) {
          let mut number = c.to_string();
          while let Some(next) = chars.peek() {
            if next.is_digit(10) {
              number.push(chars.next().unwrap());
            } else {
              break;
            }
          }

          if chars.peek() == Some(&'.') {
            number.push(chars.next().unwrap());
            while let Some(next) = chars.peek() {
              if next.is_digit(10) {
                number.push(chars.next().unwrap());
              } else {
                break;
              }
            }
            match number.parse::<f32>() {
              Ok(n) => TokenType::Double(n),
              Err(e) => {
                errors.push(format!("Invalid float number at line {}: {}", line, e));
                continue;
              }
            }
          } else {
            match number.parse::<i32>() {
              Ok(n) => TokenType::Int(n),
              Err(e) => {
                errors.push(format!("Invalid integer number at line {}: {}", line, e));
                continue;
              }
            }
          }
        } else if c.is_alphabetic() || c == '_' {
          let mut identifier = c.to_string();
          while let Some(&next) = chars.peek() {
            if next.is_alphanumeric() || next == '_' {
              identifier.push(chars.next().unwrap());
            } else {
              break;
            }
          }

          let token_type = match identifier.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "func" => TokenType::Func,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier(identifier),
          };

          tokens.push(Token {
            token_type,
            line,
            column,
          });
          continue;
        } else {
          errors.push(format!(
            "Unexpected character '{}' at line {} column {}",
            c, line, column
          ));
          continue;
        }
      }
    };

    tokens.push(Token {
      token_type,
      line,
      column,
    });
  }

  if errors.is_empty() {
    Ok(tokens)
  } else {
    Err(errors)
  }
}
