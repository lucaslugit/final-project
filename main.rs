use std::env;
use cstream::CStream;
use token::Scanner;
use crate::token::TokenType;
mod cstream;
mod token;
fn XHTML_(scanner:Scanner){
    let begin="<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/
    xhtml1-transitional.dtd\">".to_string();
    let  attributes= "<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">".to_string();
    let head="<head>";
    let head_end="</head>";
    let title="<title>";
    let title_end="</title>";
    let body="<body bgcolor=\"navy\"><font face=\"Courier New\" >";
    let body_end="</font></body>";
    let html_end="</html>";
    let p="<p style = \" color:white;  font:Courier New\"><font face=\"Courier New\" >";
    let p_end="</font></p>";
    let bold="<b>";
    let bold_end="</b>";
    let span_yellow="<span style = \" color:yellow;  \" >";
    let span_white="<span style = \" color:white;  \" ><b>";
    let span_aqua="<span style = \" color:aqua;  \" ><b>";
    let newline="<br/>";
    let space="&nbsp;";
    let span_end="</span>";
    let span_endbold="</b></span>";
    let font="<font>";
    let font_end="</font>";
    let mut H="".to_string();
    H=begin+&attributes+&head.to_string()+title+"project output file"+title_end
    +head_end+body;
    //let scan=scanner;
    for i in 0..scanner.token_vec.len(){
        let temtoken=&scanner.token_vec[i].text;
        if temtoken=="<"{
            let com = [span_white, "ⵦ"].join("");
            H+=&com;
            H+=span_endbold;
            continue;
        }
        if temtoken=="float" || temtoken=="int" || temtoken=="void" || temtoken=="main" ||
        temtoken=="while" || temtoken=="return" ||temtoken=="+" ||temtoken=="-" ||temtoken=="*" ||
        temtoken=="/" ||temtoken=="{" ||temtoken=="}" ||temtoken=="(" ||temtoken==")" ||
        temtoken==";" || temtoken=="unsigned" || temtoken=="long" || temtoken=="short"|| temtoken=="if"
        || temtoken=="=" || temtoken=="==" || temtoken=="<ffff" || temtoken==">" || temtoken=="<="
        || temtoken==">="  {
            let tem=&scanner.token_vec[i].text;
            let com = [span_white, &tem].join("");
            H+=&com;
            H+=span_endbold;
            H+=space;
        }
        else if temtoken==""{
            
            H+=space;
            
        }
        else if temtoken=="\n"&& scanner.token_vec[i+1].text!="\n" {
           
            H+=newline;
            
        }
        else if temtoken=="\n"&& scanner.token_vec[i+1].text=="\n" {
            
            continue;
        }
        else if temtoken!=""&&temtoken!="\n"{
            let check_dit=temtoken.chars().nth(0).unwrap();
            if check_dit.is_ascii_digit(){
                let com = [span_aqua, &temtoken].join("");
                H+=&com;
                H+=span_endbold;
            }
            else  {
                let com = [span_yellow, &temtoken].join("");
                H+=&com;
                H+=span_end;
            }
            

        }
      
    }
    
    H+=body_end;
    H+=html_end;
    let mut file = std::fs::File::create("data.xhtml").expect("create failed");
    file.write_all(H.as_bytes()
    
).expect("write failed");
    println!("data written to file" );
    
}
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
