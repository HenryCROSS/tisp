mod compiler;
mod parser;
mod repl;
mod scanner;

use scanner::scanner::read_str_scan;

fn main() {
  let input = "var x = 10;\nif (x > 5) { print(\"Hello, world!!!!!!!!!!!\"); }".to_string();
  match read_str_scan(input) {
    Ok(tokens) => println!("{:?}", tokens),
    Err(errors) => eprintln!("Errors: {:?}", errors),
  }
}
