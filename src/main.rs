mod token;
use token::*;

mod node;
use node::*;

mod lexer;
use lexer::*;

mod parser;
use parser::*;

mod interpreter;
use interpreter::*;

fn main() {
    println!("Hello, world!");
    println!("{:?}", tokenize("1 + 2 + 34 + 567"));
    println!("{:?}", parse(&tokenize("1 + 2 + 3 + 4 + 5 + 6").unwrap(), 0))
}
