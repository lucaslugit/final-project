use std::env;
use cstream::CStream;
use token::Scanner;
use parser::Parser;
use token::TokenType;
use token::Token;
mod cstream;
mod token;
mod parser;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
mod xhtml;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stream = CStream::new(&args[1]);
    while let Some(ch) = stream.read() {
        print!("{}", ch);
    }
    println!("");
    let filename = &args[1];
    let stream = CStream::new(&args[1]);
    let mut scanner = Scanner::new(stream);
    // ALL_TOKEN is HERE!
    let mut all_tokens = Vec::new();
    loop {
        let token = scanner.get_next_token();
        match &token.token_type {
            TokenType::Invalid(_) => {
                all_tokens.push(token);
                break;
            }
            _ => all_tokens.push(token),
        }
    }
    let parser = Parser::new(all_tokens);
    let result = parser.fn_program();
    if result.is_ok() {
        println!("all case passed");
        xhtml::Output(filename);
    } else {
        println!("{:#?}", result);
    }
    // let mut xhtml_input = Partfour {token_vec:vec_part_four,current_token_index:0};
    // XHTML_(xhtml_input);
}
