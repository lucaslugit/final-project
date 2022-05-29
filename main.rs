use std::io::Read;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;



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
   
    token_vec:Vec<Token>,
    current_token_index:i32,
    
}
impl Scanner{
    fn get_next_token(&mut self)->&Token {
      let tem=&self.token_vec[self.current_token_index as usize];
      self.current_token_index+=1;
      return tem;
    }

}
//store all token in vector for scanner
fn init_token(length:usize ,all_tokens:Vec<String>)->Vec<Token>{
    let mut all_token : Vec<Token> = Vec::new();
    for i in 0..length{
        let tem_token = Token{
           
            text : all_tokens[i].to_string(),
            token_type:String::from(""),
            line_num:0,
            char_pos:0,
        };
        all_token.push(tem_token);
    }
    return all_token;

}
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
    //println!("{}",H);
    file.write_all(H.as_bytes()
    


).expect("write failed");
    println!("data written to file" );
    
}
fn main() {
    let mut f = CStream {
        contents: String::from(""),
    };
   
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("file can not be opened!");
    
    f.contents=contents;     //CSstream store the contents of the file

    //println!("{}", f.contents);
    //all_tokens stores all tokens in file
    let mut all_tokens : Vec<String> = Vec::new();
    //println!("{}",f.contents.len());
    let mut temstring = String::from("");
    //Store all the tokens in the input program in order as a vector of tokens.
    for i in 0..f.contents.len(){
        let  tem=f.contents.chars().nth(i).unwrap();
        if tem !=' ' && tem!= '\n' {
            if tem=='(' || tem==')' ||tem==';'|| tem=='{'||tem=='}' || tem=='[' || tem==']'{
                all_tokens.push(temstring);
                temstring=String::from("");
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
        if tem=='\n'{
            temstring.push(tem);
            all_tokens.push(temstring);
            temstring=String::from("");
        }

    }
   


    //vector store all token for scanning
    let all_token=init_token(all_tokens.len(),all_tokens);
    
    let mut scanner = Scanner {
        token_vec: all_token,
        current_token_index:0,

    };
    for i in 0..scanner.token_vec.len(){
        println! ("{:?}",scanner.token_vec[i].text);
    }
    //example of return the next token as read from the .x file. The return type is Token.
   
    // for i in 0..scanner.token_vec.len(){
    //     let curr_index=scanner.current_token_index;
    //     let next_token=scanner.get_next_token();
    //     println!("{:?}",next_token.text);
       
    //     //println!("{}",curr_index);
        
   
    // };

XHTML_(scanner);

    

}
