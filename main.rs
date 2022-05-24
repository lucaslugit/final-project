use std::io::Read;
use std::env;
use std::fs;
struct CStream {
   
    contents:String,
   
}
enum TokenType { 
    
    IntConstant, 
    FloatConstant, 
    Keyword, 
    Operator,
    Identifier,
    Invalid,
}
struct Token {
   
    text : String,
    token_type:String,
    line_num:i32,
    char_pos:i32,
}
struct Scanner {
   
    token : Token,
    
}

fn main() {
    let mut f = CStream {
        contents: String::from(""),
    };
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("file can not be opened!");
    f.contents=contents;
    println!("{}", f.contents);

    let mut all_tokens : Vec<String> = Vec::new();
    println!("{}",f.contents.len());
    let mut temstring = String::from("");
    for i in 0..f.contents.len(){
        let  tem=f.contents.chars().nth(i).unwrap();
        if tem !=' ' && tem!= '\n' {
            if tem=='(' || tem==')' ||tem==';'{
                all_tokens.push(tem.to_string());
            }
            else{
                temstring.push(tem);

            }
            

        }
        if tem==' '{
            all_tokens.push(temstring);
            temstring=String::from("");
        }

    }

    for i in 0..all_tokens.len(){
        println!("{}",all_tokens[i]);
    }
}
