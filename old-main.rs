use std::env;
use cstream::CStream;
use token::Scanner;
use crate::token::TokenType;
mod cstream;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stream = CStream::new(&args[1]);
    while let Some(ch) = stream.read() {
        print!("{}", ch);
    }
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
    while let Some(mytoken) = all_tokens.pop(){
        println!("Token_TEXT is {}", mytoken.text);
        println!("Token_type is {:?}", mytoken.token_type);
    }
}
