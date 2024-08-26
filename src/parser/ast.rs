#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
  Program(Vec<ASTNode>),
  Int32(i32),
  Float32(f32),
  Bool(bool),
  Nil,
  Symbol(String),
  Keyword(String),
  StringLiteral(String),
  Character(char),
  List(Vec<ASTNode>),
  Quote(Box<ASTNode>),
  FuncCall(String, Vec<ASTNode>),
  FuncDef(String, Vec<ASTNode>, Vec<ASTNode>),
  MacroDef(String, Vec<ASTNode>, Vec<ASTNode>),
  MacroCall(String, Vec<ASTNode>),
}