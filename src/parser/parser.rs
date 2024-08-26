use super::{
  ast::ASTNode,
  parser_error::{ParseError, ParseResult},
};
use crate::scanner::token::{Token, TokenType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
  macros: HashMap<String, Vec<ASTNode>>,
  functions: HashMap<String, Vec<ASTNode>>,
  errors: Vec<ParseError>,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      current: 0,
      macros: HashMap::new(),
      functions: HashMap::new(),
      errors: Vec::new(),
    }
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.tokens.len()
  }

  fn peek(&self) -> Option<&Token> {
    self.tokens.get(self.current)
  }

  fn advance(&mut self) -> Option<&Token> {
    if !self.is_at_end() {
      self.current += 1;
    }
    self.previous()
  }

  fn previous(&self) -> Option<&Token> {
    if self.current > 0 {
      self.tokens.get(self.current - 1)
    } else {
      None
    }
  }

  fn is_current_match(&self, token_type: &TokenType) -> bool {
    match self.peek() {
      Some(token) => &token.token_type == token_type,
      None => false,
    }
  }

  fn match_tokens(&mut self, types: &[TokenType]) -> bool {
    if let Some(token) = self.peek() {
      if types.iter().any(|t| t == &token.token_type) {
        self.advance();
        return true;
      }
    }
    false
  }

  pub fn parse(&mut self) -> Result<ASTNode, Vec<ParseError>> {
    let mut nodes = Vec::new();

    while !self.is_at_end() {
      if self.match_tokens(&[
        TokenType::LeftParen,
        TokenType::Keyword("macro".to_string()),
      ]) {
        match self.parse_symbol() {
          Ok(name) => match self.parse_macro_definition(name) {
            Ok(node) => nodes.push(node),
            Err(err) => self.errors.push(err),
          },
          Err(err) => self.errors.push(err),
        }
      } else {
        match self.parse_expression() {
          Ok(node) => nodes.push(node),
          Err(err) => self.errors.push(err),
        }
      }
    }

    if self.errors.is_empty() {
      Ok(ASTNode::Program(nodes))
    } else {
      Err(self.errors.clone())
    }
  }

  fn parse_expression(&mut self) -> ParseResult<ASTNode> {
    if self.match_tokens(&[TokenType::LeftParen]) {
      self.parse_list()
    } else {
      self.parse_atom()
    }
  }

  fn parse_list(&mut self) -> ParseResult<ASTNode> {
    let mut elements = Vec::new();

    while !self.is_current_match(&TokenType::RightParen) && !self.is_at_end() {
      if self.match_tokens(&[TokenType::Symbol("def".to_string())]) {
        let name = self.parse_symbol()?;
        let definition = self.parse_definition(name)?;
        elements.push(definition);
      } else if self.match_tokens(&[TokenType::Keyword("quote".to_string())]) {
        let quoted_expr = self.parse_expression()?;
        elements.push(ASTNode::Quote(Box::new(quoted_expr)));
      } else {
        elements.push(self.parse_expression()?);
      }
    }

    if !self.is_current_match(&TokenType::RightParen) {
      return Err(self.error("Expected ')' at the end of list"));
    }

    self.advance(); // Consume ')'
    Ok(ASTNode::List(elements))
  }

  fn parse_definition(&mut self, name: String) -> ParseResult<ASTNode> {
    if self.match_tokens(&[TokenType::LeftParen]) {
      if self.match_tokens(&[TokenType::Symbol("fn".to_string())]) {
        self.parse_function_definition(name)
      } else {
        Err(self.error("Expected 'fn' after 'def'"))
      }
    } else {
      self.parse_atom() // 如果没有函数定义，则直接解析为原子
    }
  }

  fn parse_function_definition(&mut self, name: String) -> ParseResult<ASTNode> {
    let params = self.parse_arg_list()?; // 解析函数的参数列表
    let mut body = Vec::new();

    while !self.is_current_match(&TokenType::RightParen) && !self.is_at_end() {
      body.push(self.parse_expression()?);
    }

    if !self.is_current_match(&TokenType::RightParen) {
      return Err(self.error("Expected ')' to close function body"));
    }

    self.advance(); // Consume ')'
    Ok(ASTNode::FuncDef(name, params, body))
  }

  fn parse_macro_definition(&mut self, name: String) -> ParseResult<ASTNode> {
    let params = self.parse_arg_list()?;
    let mut body = Vec::new();

    while !self.is_current_match(&TokenType::RightParen) && !self.is_at_end() {
      body.push(self.parse_expression()?);
    }

    if !self.is_current_match(&TokenType::RightParen) {
      return Err(self.error("Expected ')' to close macro body"));
    }

    self.advance(); // Consume ')'
    Ok(ASTNode::MacroDef(name, params, body))
  }

  fn parse_arg_list(&mut self) -> ParseResult<Vec<ASTNode>> {
    if !self.is_current_match(&TokenType::LeftParen) {
      return Err(self.error("Expected '(' to start argument list"));
    }
    self.advance(); // Consume '('
    let mut params = Vec::new();

    while !self.is_current_match(&TokenType::RightParen) && !self.is_at_end() {
      let param = self.parse_symbol()?;
      params.push(ASTNode::Symbol(param));
    }

    if !self.is_current_match(&TokenType::RightParen) {
      return Err(self.error("Expected ')' to close argument list"));
    }

    self.advance(); // Consume ')'
    Ok(params)
  }

  fn parse_atom(&mut self) -> ParseResult<ASTNode> {
    let token = match self.advance() {
      Some(token) => token.clone(),
      None => return Err(self.error("Unexpected end of input")),
    };

    match token.token_type {
      TokenType::Int32(value) => Ok(ASTNode::Int32(value)),
      TokenType::Float32(value) => Ok(ASTNode::Float32(value)),
      TokenType::Bool(value) => Ok(ASTNode::Bool(value)),
      TokenType::Symbol(value) => Ok(ASTNode::Symbol(value)),
      TokenType::Keyword(value) => Ok(ASTNode::Keyword(value)),
      TokenType::String(value) => Ok(ASTNode::StringLiteral(value)),
      TokenType::Character(value) => Ok(ASTNode::Character(value)),
      TokenType::Quote => {
        let expr = self.parse_expression()?;
        Ok(ASTNode::Quote(Box::new(expr)))
      }
      _ => Err(self.error("Unexpected token")),
    }
  }

  fn parse_symbol(&mut self) -> ParseResult<String> {
    match self.advance() {
      Some(Token {
        token_type: TokenType::Symbol(name),
        ..
      }) => Ok(name.clone()),
      _ => Err(self.error("Expected symbol")),
    }
  }

  fn error(&self, message: &str) -> ParseError {
    if let Some(token) = self.peek() {
      ParseError::new(message, token.line, token.column)
    } else {
      ParseError::new(message, 0, 0)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::scanner::scanner::read_str_scan;

  use super::*;

  fn parse_lisp_code(code: &str) -> Result<ASTNode, Vec<ParseError>> {
    let tokens = read_str_scan(code.to_string()).unwrap();
    let mut parser = Parser::new(tokens);
    parser.parse()
  }

  #[test]
  fn test_simple_function_definition() {
    let code = r#"
            (def add 
              (fn (x y)
                (+ x y)))
        "#;

    let result = parse_lisp_code(code);

    println!("????{:?}", result);
    assert!(result.is_ok());

    let expected_ast = ASTNode::Program(vec![ASTNode::FuncDef(
      "add".to_string(),
      vec![
        ASTNode::Symbol("x".to_string()),
        ASTNode::Symbol("y".to_string()),
      ],
      vec![ASTNode::List(vec![
        ASTNode::Symbol("+".to_string()),
        ASTNode::Symbol("x".to_string()),
        ASTNode::Symbol("y".to_string()),
      ])],
    )]);

    assert_eq!(result.unwrap(), expected_ast);
  }

  #[test]
  fn test_macro_definition() {
    let code = r#"
            (macro log (msg)
                `(println ,msg)
            )
        "#;

    let result = parse_lisp_code(code);

    assert!(result.is_ok());

    let expected_ast = ASTNode::Program(vec![ASTNode::MacroDef(
      "log".to_string(),
      vec![ASTNode::Symbol("msg".to_string())],
      vec![ASTNode::Quote(Box::new(ASTNode::List(vec![
        ASTNode::Symbol("println".to_string()),
        ASTNode::Symbol("msg".to_string()),
      ])))],
    )]);

    assert_eq!(result.unwrap(), expected_ast);
  }

  #[test]
  fn test_function_call() {
    let code = r#"
            (add 10 20)
        "#;

    let result = parse_lisp_code(code);

    assert!(result.is_ok());

    let expected_ast = ASTNode::Program(vec![ASTNode::List(vec![
      ASTNode::Symbol("add".to_string()),
      ASTNode::Int32(10),
      ASTNode::Int32(20),
    ])]);

    assert_eq!(result.unwrap(), expected_ast);
  }

  #[test]
  fn test_nested_expressions() {
    let code = r#"
            (def calculate (fn (a b c)
                (+ a (* b c))
            ))
        "#;

    let result = parse_lisp_code(code);

    assert!(result.is_ok());

    let expected_ast = ASTNode::Program(vec![ASTNode::FuncDef(
      "calculate".to_string(),
      vec![
        ASTNode::Symbol("a".to_string()),
        ASTNode::Symbol("b".to_string()),
        ASTNode::Symbol("c".to_string()),
      ],
      vec![ASTNode::List(vec![
        ASTNode::Symbol("+".to_string()),
        ASTNode::Symbol("a".to_string()),
        ASTNode::List(vec![
          ASTNode::Symbol("*".to_string()),
          ASTNode::Symbol("b".to_string()),
          ASTNode::Symbol("c".to_string()),
        ]),
      ])],
    )]);

    assert_eq!(result.unwrap(), expected_ast);
  }

  #[test]
  fn test_quote_expression() {
    let code = r#"
            '(1 2 3)
        "#;

    let result = parse_lisp_code(code);

    assert!(result.is_ok());

    let expected_ast = ASTNode::Program(vec![ASTNode::Quote(Box::new(ASTNode::List(vec![
      ASTNode::Int32(1),
      ASTNode::Int32(2),
      ASTNode::Int32(3),
    ])))]);

    assert_eq!(result.unwrap(), expected_ast);
  }

  #[test]
  fn test_parsing_errors() {
    let code = r#"
            (def incomplete-fn (fn (x y)
                (+ x y)
        "#;

    let result = parse_lisp_code(code);

    assert!(result.is_err());

    let errors = result.unwrap_err();

    // 验证错误是否累积到 Vec 中
    assert_eq!(errors.len(), 1);
    assert!(errors[0]
      .message
      .contains("Expected ')' to close function body"));
  }
}
