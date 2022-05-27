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
    println!("{}",f.contents.len());
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
    //example of return the next token as read from the .x file. The return type is Token.
   
    for i in 0..scanner.token_vec.len(){
        let curr_index=scanner.current_token_index;
        let next_token=scanner.get_next_token();
        println!("{:?}",next_token.text);
        println!("{}",curr_index);
        
   
    };

   
    

}
