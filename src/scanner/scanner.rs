use super::token::{Token, TokenType};
use std::fs;

pub fn read_file_scan(file_name: String) -> Result<Vec<Token>, Vec<String>> {
  fs::read_to_string(file_name)
    .map_err(|e| vec![e.to_string()])
    .and_then(read_str_scan)
}

pub fn read_str_scan(text: String) -> Result<Vec<Token>, Vec<String>> {
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
      '/' => TokenType::Symbol("/".to_string()),
      '*' => TokenType::Symbol("*".to_string()),
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
          TokenType::Symbol("==".to_string())
        } else {
          TokenType::Symbol("=".to_string())
        }
      }
      '>' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          TokenType::Symbol(">=".to_string())
        } else {
          TokenType::Symbol(">".to_string())
        }
      }
      '<' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          TokenType::Symbol("<=".to_string())
        } else {
          TokenType::Symbol("<".to_string())
        }
      }
      '\'' => TokenType::Quote,
      ':' => {
        let mut keyword = String::new();
        while let Some(&next) = chars.peek() {
          if next.is_whitespace() || next == ')' || next == '(' {
            break;
          }
          keyword.push(chars.next().unwrap());
        }
        TokenType::Keyword(keyword)
      }
      '"' => {
        let mut string_literal = String::new();
        let start_column = column; // 记录字符串开始的列号
        let mut unterminated = false;

        while let Some(&next) = chars.peek() {
          if next == '"' {
            chars.next(); // 消耗结束的引号
            break;
          } else if next == '\\' {
            // 处理转义字符
            chars.next(); // 消耗 '\'
            if let Some(&escaped_char) = chars.peek() {
              match escaped_char {
                '"' => string_literal.push('"'),
                'n' => string_literal.push('\n'),
                't' => string_literal.push('\t'),
                '\\' => string_literal.push('\\'),
                _ => {
                  errors.push(format!(
                    "Unknown escape sequence \\{} at line {}, column {}",
                    escaped_char, line, column
                  ));
                }
              }
              chars.next(); // 消耗转义后的字符
              column += 1;
            } else {
              errors.push(format!(
                "Incomplete escape sequence at line {}, column {}",
                line, column
              ));
              break;
            }
          } else if next == '\n' {
            unterminated = true; // 标记为未终止
            break;
          } else {
            string_literal.push(chars.next().unwrap());
            column += 1; // 更新列号
          }
        }

        if unterminated || chars.peek().is_none() {
          errors.push(format!(
            "Unterminated string starting at line {}, column {}",
            line, start_column
          ));
        }

        TokenType::String(string_literal)
      }
      '#' => {
        if chars.peek() == Some(&'t') {
          chars.next();
          TokenType::Bool(true)
        } else if chars.peek() == Some(&'f') {
          chars.next();
          TokenType::Bool(false)
        } else if chars.peek() == Some(&'\\') {
          chars.next();
          if let Some(&next) = chars.peek() {
            chars.next();
            TokenType::Character(next)
          } else {
            errors.push(format!(
              "Invalid character literal at line {} column {}",
              line, column
            ));
            continue;
          }
        } else {
          errors.push(format!(
            "Unexpected character '{}' at line {} column {}",
            c, line, column
          ));
          continue;
        }
      }
      '`' | ',' => TokenType::Symbol(c.to_string()),
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
              Ok(n) => TokenType::Float32(n),
              Err(e) => {
                errors.push(format!("Invalid float number at line {}: {}", line, e));
                continue;
              }
            }
          } else {
            match number.parse::<i32>() {
              Ok(n) => TokenType::Int32(n),
              Err(e) => {
                errors.push(format!("Invalid integer number at line {}: {}", line, e));
                continue;
              }
            }
          }
        } else if c.is_alphabetic() || c == '_' || "+-*/><=!?".contains(c) {
          let mut identifier = c.to_string();
          while let Some(&next) = chars.peek() {
            if next.is_alphanumeric() || next == '_' || "+-*/><=!?".contains(next) {
              identifier.push(chars.next().unwrap());
            } else {
              break;
            }
          }

          let token_type = match identifier.as_str() {
            "def" => TokenType::Var,
            "fn" => TokenType::Func,
            "macro" => TokenType::Macro,
            "quote" => TokenType::Quote,
            "true" => TokenType::Bool(true),
            "false" => TokenType::Bool(false),
            _ => TokenType::Symbol(identifier),
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_symbols_and_keywords() {
    let input = "(def add (fn (x y) (+ x y)))".to_string();
    let result = read_str_scan(input);

    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_eq!(tokens.len(), 16);

    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Var);
    assert_eq!(tokens[2].token_type, TokenType::Symbol("add".to_string()));
    assert_eq!(tokens[3].token_type, TokenType::LeftParen);
    assert_eq!(tokens[4].token_type, TokenType::Func);
    assert_eq!(tokens[5].token_type, TokenType::LeftParen);
    assert_eq!(tokens[6].token_type, TokenType::Symbol("x".to_string()));
    assert_eq!(tokens[7].token_type, TokenType::Symbol("y".to_string()));
    assert_eq!(tokens[8].token_type, TokenType::RightParen);
    assert_eq!(tokens[9].token_type, TokenType::LeftParen);
    assert_eq!(tokens[10].token_type, TokenType::Symbol("+".to_string()));
    assert_eq!(tokens[11].token_type, TokenType::Symbol("x".to_string()));
    assert_eq!(tokens[12].token_type, TokenType::Symbol("y".to_string()));
    assert_eq!(tokens[13].token_type, TokenType::RightParen);
    assert_eq!(tokens[14].token_type, TokenType::RightParen);
    assert_eq!(tokens[15].token_type, TokenType::RightParen);
  }

  #[test]
  fn test_strings_and_keywords() {
    let input = r#"(def str "Hello, World!")"#.to_string();
    let result = read_str_scan(input);

    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_eq!(tokens.len(), 5);

    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Var);
    assert_eq!(tokens[2].token_type, TokenType::Symbol("str".to_string()));
    assert_eq!(
      tokens[3].token_type,
      TokenType::String("Hello, World!".to_string())
    );
    assert_eq!(tokens[4].token_type, TokenType::RightParen);
  }

  #[test]
  fn test_numbers_and_booleans() {
    let input = "(def values (list 42 3.14 #t #f))".to_string();
    let result = read_str_scan(input);

    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_eq!(tokens.len(), 11);

    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Var);
    assert_eq!(
      tokens[2].token_type,
      TokenType::Symbol("values".to_string())
    );
    assert_eq!(tokens[3].token_type, TokenType::LeftParen);
    assert_eq!(tokens[4].token_type, TokenType::Symbol("list".to_string()));
    assert_eq!(tokens[5].token_type, TokenType::Int32(42));
    assert_eq!(tokens[6].token_type, TokenType::Float32(3.14));
    assert_eq!(tokens[7].token_type, TokenType::Bool(true));
    assert_eq!(tokens[8].token_type, TokenType::Bool(false));
    assert_eq!(tokens[9].token_type, TokenType::RightParen);
    assert_eq!(tokens[10].token_type, TokenType::RightParen);
  }

  #[test]
  fn test_string_with_escape_sequences() {
    let input = r#"(print "said \"d\" and then \n new line")"#.to_string();
    let result = read_str_scan(input);

    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_eq!(tokens.len(), 4);

    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Symbol("print".to_string()));
    assert_eq!(
      tokens[2].token_type,
      TokenType::String("said \"d\" and then \n new line".to_string())
    );
    assert_eq!(tokens[3].token_type, TokenType::RightParen);
  }

  #[test]
  fn test_character_literal() {
    let input = r"(def char #\a)".to_string();
    let result = read_str_scan(input);

    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_eq!(tokens.len(), 5);

    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Var);
    assert_eq!(tokens[2].token_type, TokenType::Symbol("char".to_string()));
    assert_eq!(tokens[3].token_type, TokenType::Character('a'));
    assert_eq!(tokens[4].token_type, TokenType::RightParen);
  }

  #[test]
  fn test_unterminated_string_error() {
    let input = r#"(print "Hello, World)"#.to_string();
    let result = read_str_scan(input);

    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("Unterminated string"));
  }

  #[test]
  fn test_unexpected_character_error() {
    let input = "(def x @)".to_string();
    let result = read_str_scan(input);

    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("Unexpected character '@'"));
  }

  #[test]
  fn test_macro_definition() {
    let input = "(macro log (msg) `(println ,msg))".to_string(); // 修改后的输入
    let result = read_str_scan(input);

    println!("Result: {:?}", result); // 用于调试输出

    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_eq!(tokens.len(), 13);

    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Macro); // "macro" 关键字
    assert_eq!(tokens[2].token_type, TokenType::Symbol("log".to_string())); // 宏名称
    assert_eq!(tokens[3].token_type, TokenType::LeftParen);
    assert_eq!(tokens[4].token_type, TokenType::Symbol("msg".to_string())); // 参数名称
    assert_eq!(tokens[5].token_type, TokenType::RightParen);
    assert_eq!(tokens[6].token_type, TokenType::Symbol("`".to_string())); // Quasiquote (`)
    assert_eq!(tokens[7].token_type, TokenType::LeftParen);
    assert_eq!(
      tokens[8].token_type,
      TokenType::Symbol("println".to_string())
    ); // "println" 函数调用
    assert_eq!(tokens[9].token_type, TokenType::Symbol(",".to_string())); // Unquote (`,`)
    assert_eq!(tokens[10].token_type, TokenType::Symbol("msg".to_string())); // 参数引用
    assert_eq!(tokens[11].token_type, TokenType::RightParen);
    assert_eq!(tokens[12].token_type, TokenType::RightParen);
  }
}
