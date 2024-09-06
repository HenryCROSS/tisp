use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
  Program(Vec<ASTNode>),
  Int32(i32),
  Float32(f32),
  Bool(bool),        // #t #f
  Nil,               // nil
  Symbol(String),    // symbol
  Keyword(String),   // :keyword
  StringLiteral(String),
  Character(char),
  List(Vec<ASTNode>),
  Quote(Box<ASTNode>),
  Variable(String, Box<ASTNode>),
  FuncDef(Vec<ASTNode>, Vec<ASTNode>),
  MacroDef(String),
  MacroTemplate(Uuid),
  MacroComma(Box<ASTNode>),
  MacroListExpand(Box<ASTNode>),
}