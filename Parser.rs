use custom_error::custom_error;

custom_error!{ pub SyntaxError
{line_num: i32 , pos :i32} = "Error at Line {line_num} Character {pos}."

}
use std::clone;

#[derive(Clone)]
pub enum TokenType { 
    IntConstant, 
    FloatConstant, 
    Keyword, 
    Operator,
    Identifier,
    Invalid,
}


pub struct Token {
   
    pub text : String,
    pub token_type:TokenType,
    pub line_num:i32,
    pub char_pos:i32,
}

pub struct Parser{
    input:Vec<Token>,
    token_pos:usize,
    token_match:bool
}

impl Parser{
    pub fn new(all_tokens:Vec<Token>) -> Parser{
        Parser{
            input:all_tokens,
            token_pos: 0,
            token_match: true
        }
    }

    pub fn get_next_token(&mut self) -> bool {
        if self.token_pos >= self.input.len(){
            eprintln!("{}",SyntaxError{line_num:self.input[self.line_num],pos:self.input[self.char_pos]}.to_string());
            return false
        }
        else{
            println!("{}",self.input[self.token_pos].text);
            self.token_pos = self.token_pos + 1;
            return true
        }
    }

    pub fn func_MultOperator(&mut self) -> bool{
        if self.get_next_token(self) == true {
            if self.input[self.token_pos-1].text == "*" || self.input[self.token_pos-1].text == "/"{
                self.token_match = true;
                return true
            }
            else{
                eprintln!("{}",SyntaxError{line_num:self.input[self.line_num],pos:self.input[self.char_pos]}.to_string());
                return false
            }
        }
        return true
    }

    pub fn func_AddOperator(&mut self) -> bool {
        if self.get_next_token(self) == true {
            if self.input[self.token_pos-1].text == "+" || self.input[self.token_pos-1].text == "-"{
                self.token_match = true;
                return true
            }
            else{
                eprintln!("{}",SyntaxError{line_num:self.input[self.line_num],pos:self.input[self.char_pos]}.to_string());
                return false
            }
        }
        return true
    }

    pub fn func_RelationOperator(&mut self) -> bool {
        if self.get_next_token(self) == true {
            if self.input[self.token_pos-1].text == "==" || self.input[self.token_pos-1].text == "<" || self.input[self.token_pos-1].text == ">"|| self.input[self.token_pos-1].text == "<="|| self.input[self.token_pos-1].text == ">=" || self.input[self.token_pos-1].text == "!="{
                self.token_match = true;
                return true
            }
            else{
                return false
            }
        }
        return true
    }

    pub fn factor(&mut self) -> bool{
        if self.get_next_token(self) == true {
            if self.input[self.token_pos-1].text == "("{
                self.token_match = true;
                if self.get_next_token(self) == false{
                    eprintln!("{}",SyntaxError{line_num:self.input[self.line_num],pos:self.input[self.char_pos]}.to_string());
                } 
                if expression(self) == true {
                    if self.get_next_token(self) == true {
                        if self.input[self.token_pos-1].text == ")"{
                            self.token_match = true;
                            return true
                        }
                        else{
                            eprintln!("{}",SyntaxError{line_num:self.input[self.line_num],pos:self.input[self.char_pos]}.to_string());
                        }
                    }
                    else{
                        eprintln!("{}",SyntaxError{line_num:self.input[self.line_num],pos:self.input[self.char_pos]}.to_string());
                    }
                }
                else{
                    eprintln!("{}",SyntaxError{line_num:self.input[self.line_num],pos:self.input[self.char_pos]}.to_string());
                }
            }
        }
    }

}