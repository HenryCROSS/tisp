mod compiler;
mod parser;
mod repl;
mod scanner;

fn main() {
  let op = compiler::opcode::Opcode::ADD;
  let bytes = compiler::opcode::opcode_to_bytes(op);
  println!("{:02X?}", bytes);
}
