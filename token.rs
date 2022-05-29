use super::cstream::CStream;

#[derive(Debug)]
pub enum TokenType {
    IntConstant(i64),
    FloatConstant(f64),
    Keyword(String),
    Operator(String),
    Identifier(String),
    Invalid(String),
}

//decide the TokenType and store the token_str for future use 
impl TokenType {
    pub fn from(token_str: &str) -> TokenType {
        if let Ok(i) = token_str.parse::<i64>() {
            TokenType::IntConstant(i)
        } else if let Ok(f) = token_str.parse::<f64>() {
            TokenType::FloatConstant(f)
        } else {
            match token_str {
                "unsigned" | "char" | "short" | "int" | "long" | "float" | "double" | "while"
                | "if" | "return" | "void" | "main" => TokenType::Keyword(token_str.to_string()),
                "(" | ")" | "{" | "}" | "," | "=" | "==" | "<" | ">" | "<=" | ">=" | "!=" | "+"
                | "-" | "*" | "/" | ";" => TokenType::Operator(token_str.to_string()),
                _ => {
                    let mut is_var = true;
                    for ch in token_str.chars() {
                        //check if the token could be a var if it start with numericalnumber then it is not a vaild identifier
                        if ch.is_alphanumeric() == false {
                            is_var = false;
                            break;
                        }
                    }
                    if is_var && !token_str.is_empty() {
                        TokenType::Identifier(token_str.to_string())
                    } else {
                        TokenType::Invalid(token_str.to_string())
                    }
                }
            }
        }
    }
}

pub struct Scanner {
    stream: CStream,
    line_num: usize,
    char_pos: usize,
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    line_num: i32,
    char_pos: i32,
}

impl Scanner {
    pub fn new(stream: CStream) -> Scanner {
        Scanner {
            stream,
            line_num: 0,
            char_pos: 0,
        }
    }
    //get next char
    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.stream.read() {
            if ch == '\n' {
                self.line_num += 1;
                self.char_pos = 0;
            } else {
                self.char_pos += 1;
            };
            Some(ch)
        } else {
            None
        }
    }
    fn look_curr(&self) -> Option<char> {
        self.stream.look_curr()
    }
    pub fn get_next_token(&mut self) -> Token {
        //check if there is next char and jump over space and \n
        while let Some(ch) = self.look_curr() {
            if ch == ' ' || ch == '\n' {
                self.next();
            } else {
                break;
            }
        }
        let line_num = self.line_num as i32;
        let char_pos = self.char_pos as i32;
        let mut chars = Vec::new();

        let mut token = Token {
            text: String::new(),
            token_type: TokenType::from(""),
            line_num,
            char_pos,
        };
        while let Some(ch) = self.look_curr() {
            chars.push(ch);
            let text = chars.iter().collect::<String>();
            let token_type = TokenType::from(&text);
            match token.token_type {
                TokenType::Invalid(_) => {} // invaild tokentype
                _ => match token_type { 
                    TokenType::Invalid(_) => {
                        break;
                    }
                    _ => {}
                },
            }
            token = Token {
                text,
                token_type,
                line_num,
                char_pos,
            };
            self.next();
        }
        token
    }
}
