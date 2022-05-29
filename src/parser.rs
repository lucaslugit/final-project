// use crate::custom_error;

// use custom_error::custom_error;
// custom_error!{ 
//     pub SyntaxError {line_num: i32 , pos :i32, rule: String} = "Error at Line {line_num} Character {pos}.  The syntax should be: {rule}"
// }

use crate::token::{Token, TokenType};
#[derive(Debug)]
pub struct Parser{
    all_tokens:Vec<Token>,
    vec_pos:usize
}

impl Parser{
    pub fn new(all_tokens:Vec<Token>) -> Parser{
        Parser{
            all_tokens:all_tokens,
            vec_pos:0
        }
    }

    fn fn_mult(&self, pos: usize) -> Result <usize,String>{
        let i = pos;
        if self.all_tokens[i].token_type == TokenType::Operator(String::from("*")) || self.all_tokens[i].token_type == TokenType::Operator(String::from("/")){ //each token's text is stored in corresponding enum while making its tokentype. so we can just compare tokentype(token.text) to match both tokentype and token.text
            Ok(i + 1)
        }
        else{
            Err(format!(
                "Error at Line {} Character {}. The syntax should be: MultOperator := * | / ",
                self.all_tokens[i].line_num, self.all_tokens[i].char_pos
            ))
        }
    }

    fn fn_add(&self, pos: usize) -> Result<usize, String> {
        let i = pos;
        if self.all_tokens[i].token_type == TokenType::Operator(String::from("+"))
            || self.all_tokens[i].token_type == TokenType::Operator(String::from("-"))
        {
            Ok(i + 1)
        } else {
            Err(format!(
                "Error at Line {} Character {}. The syntax should be: AddOperator := + | - ",
                self.all_tokens[i].line_num, self.all_tokens[i].char_pos
            ))
        }
    }

    fn fn_relation(&self, pos: usize) -> Result<usize, String> {
        let i = pos;
        match &self.all_tokens[i].token_type {
            TokenType::Operator(op) => match op.as_str() {
                "==" | "<" | ">" | "<=" | ">=" | "!=" => Ok(i+1),
                _ =>  Err(format!("Error at Line {} Character {}. The syntax should be: RelationOperator := == | < | > | <= | >= | !=", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
            },
            _ =>  Err(format!("Error at Line {} Character {}. The syntax should be: RelationOperator := == | < | > | <= | >= | !=", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
        }
    }

    fn fn_factor(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        if let Ok(end) = self.fn_constant(i) { // Constant situation
            Ok(end)
        } else { 
            match &self.all_tokens[i].token_type {
                TokenType::Operator(op) => { //(Expression)
                    if op == "(" {
                        i += 1;
                        i = self.fn_exp(i)?; //check if fn_exp is right - return ok(index)
                        //each expressions had its own index+1 while return ok() ... so I don't need to index+1 here..
                        if self.all_tokens[i].token_type == TokenType::Operator(String::from(")")) {
                            return Ok(i + 1);
                        }
                    }
                }
                TokenType::Identifier(id) => { // Identifier [([Expression{,Expression}])]
                    i += 1;
                    match &self.all_tokens[i].token_type {
                        TokenType::Operator(op) => { // open (
                            if op == "(" {
                                i += 1;
                                if let Ok(end) = self.fn_exp(i) { //Expression
                                    i = end;
                                    while self.all_tokens[i].token_type
                                        == TokenType::Operator(String::from(",")) // , loop to check repition{}
                                    {
                                        i = self.fn_exp(i+1)?;//Expression}
                                    }
                                }
                                if self.all_tokens[i].token_type
                                    == TokenType::Operator(String::from(")")) //close )
                                {
                                    return Ok(i + 1);
                                } else { //open ( but did not close )
                                    return Err(format!("Error at Line {} Character {}. The syntax should be: Declaration := Factor := ((Expression))|Constant|(Identifier[([Expression{{,Expression}}])]", self.all_tokens[i].line_num, self.all_tokens[i].char_pos));
                                }
                            }
                        }
                        _ => (),
                    }
                    return Ok(i); // index is inc already when I check "("
                }
                _ => (),
            }
            Err(format!("Error at Line {} Character {}. The syntax should be: Declaration := Factor := ((Expression))|Constant|(Identifier[([Expression{{,Expression}}])]", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
        }
    }

    fn fn_term(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        i = self.fn_factor(i)?; // Factor check
        while let Ok(end) = self.fn_mult(i) { //{} keep going until fn_mult fail
            if let Ok(new_end) = self.fn_factor(end){ //check factor follows mult
                i = new_end;
                continue;
            }
            else{ //if there is mult but not factor, error occurs
                return Err(format!("Error at Line {} Character {}. The syntax should be: Term := Factor {{MultOperator Factor}}", self.all_tokens[i].line_num, self.all_tokens[i].char_pos));
            }
        }// optional
        Ok(i) 
    }

    fn fn_simple_exp(&self, pos: usize) -> Result<usize, String> { 
        let mut i = pos;
        i = self.fn_term(i)?; //Term
        while let Ok(mut end) = self.fn_add(i) { //{AddOperator}
            if let Ok(new_end) = self.fn_term(end){ //{Addterm}
                i = new_end;
                continue;
            }
            else{ //no term follows add
                return Err(format!("Error at Line {} Character {}. The syntax should be: tttttt", self.all_tokens[i].line_num, self.all_tokens[i].char_pos));
            }
        }//optional
        Ok(i)
    }

    fn fn_exp(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        i = self.fn_simple_exp(pos)?;
        if let Ok(end) = self.fn_relation(i) {
            if let Ok(new_end) = self.fn_simple_exp(end){ //{Addterm}
                i = new_end;
            }
            else{ //no term follows add
                return Err(format!("Error at Line {} Character {}. The syntax should be: Expression := SimpleExpression [RelationOperator SimpleExpression]", self.all_tokens[i].line_num, self.all_tokens[i].char_pos));
            }            
        }
        Ok(i)
    }

     fn fn_return(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        if self.all_tokens[i].token_type == TokenType::Keyword(String::from("return")) { //if the token is "return"
            i += 1;
            while self.all_tokens[i].token_type != TokenType::Operator(String::from(";")) { //check end with ;
                if let Ok(end) = self.fn_exp(i){ //check expression
                    i = end;
                }
                // return nothing is ok
                // else{
                //     return Err(format!( //return 
                //         "Error at Line {} Character {}. The syntax should be: ReturnStatement := Expression ;",
                //         self.all_tokens[i].line_num, self.all_tokens[i].char_pos));
                // }
            }
            return Ok(i + 1);
        }
        Err(format!(
            "Error at Line {} Character {}. The syntax should be: ReturnStatement := Expression ;",
            self.all_tokens[i].line_num, self.all_tokens[i].char_pos
        ))
    }

    fn fn_if(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        if self.all_tokens[i].token_type == TokenType::Keyword(String::from("if")) { //check if
            i += 1;
            if self.all_tokens[i].token_type == TokenType::Operator(String::from("(")) { // check open (
                i += 1;
                i = self.fn_exp(i)?;//check expression
                if self.all_tokens[i].token_type == TokenType::Operator(String::from(")")) { //check close )
                    return self.fn_block(i + 1);//everything in block is optional
                }
            }
        }
        Err(format!("Error at Line {} Character {}. The syntax should be: IfStatement := if ( Expression ) Block", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
    }

    fn fn_while(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        if self.all_tokens[i].token_type == TokenType::Keyword(String::from("while")) { //while
            i += 1;
            if self.all_tokens[i].token_type == TokenType::Operator(String::from("(")) { //(
                i += 1;
                i = self.fn_exp(i)?; //expression
                if self.all_tokens[i].token_type == TokenType::Operator(String::from(")")) { // )
                    return self.fn_block(i + 1); //block
                }
            }
        }
        Err(format!("Error at Line {} Character {}. The syntax should be: WhileLoop := while ( Expression ) Block", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
    }

    fn fn_assg(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        match &self.all_tokens[i].token_type {
            TokenType::Identifier(_) => { // check id
                i += 1;
                if self.all_tokens[i].token_type == TokenType::Operator(String::from("=")) { //=
                    i += 1;
                    loop { //repetition
                        match &self.all_tokens[i].token_type {
                            TokenType::Identifier(_) => {
                                i += 1;
                                if self.all_tokens[i].token_type
                                    == TokenType::Operator(String::from("=")) //=
                                {
                                    i += 1;
                                } else { 
                                    i -= 1; //go back to previous token
                                    break //no rep here, go check exp
                                    // return Err(format!("Error at Line {} Character {}. The syntax should be: Assignment := Identifier = {{ Identifier = }} Expression ;", self.all_tokens[i].line_num, self.all_tokens[i].char_pos));
                                    
                                }
                            }
                            _ => break,
                        }
                    }
                    i = self.fn_exp(i)?;//check exp
                    if self.all_tokens[i].token_type == TokenType::Operator(String::from(";")) { //check end;
                        return Ok(i + 1);
                    }
                } else {
                    return Err(format!("Error at Line {} Character {}. The syntax should be: Assignment := Identifier = {{ Identifier = }} Expression ;", self.all_tokens[i].line_num, self.all_tokens[i].char_pos));
                }
            }
            _ => (), // do nothing than error
        }
        Err(format!("Error at Line {} Character {}. The syntax should be: Assignment := Identifier = {{ Identifier = }} Expression ;", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
    }

    fn fn_floattype(&self, pos: usize) -> Result<usize, String> {
        let i = pos;
        match &self.all_tokens[i].token_type {
            TokenType::Keyword(keyword) => match keyword.as_str() { // is keyword?
                "float" | "double" => Ok(i+1), // is float or double?
                _ =>  Err(format!("Error at Line {} Character {}. The syntax should be: FloatType := float | double ", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
            },
            _ =>  Err(format!("Error at Line {} Character {}. The syntax should be: FloatType := float | double ", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
        }
    }

    fn fn_inttype(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        if self.all_tokens[i].token_type == TokenType::Keyword(String::from("unsigned")) { //optional unsigned
            i += 1;
        }
        match &self.all_tokens[i].token_type {
            TokenType::Keyword(keyword) => match keyword.as_str() {  // is keyword?
                "char" | "short" | "int" | "long" => Ok(i+1), //is "char" | "short" | "int" | "long"?
                _ =>  Err(format!("Error at Line {} Character {}. The syntax should be: IntegerType := [unsigned] ( char | short | int | long ) ", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
            },
            _ =>  Err(format!("Error at Line {} Character {}. The syntax should be: IntegerType := [unsigned] ( char | short | int | long ) ", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
        }
    }

    fn fn_para(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        i = self.fn_datatype(i)?; //check datatype
        if let TokenType::Identifier(_) = self.all_tokens[i].token_type { //identifier?
            Ok(i+1)
        } else {
            Err(format!("Error at Line {} Character {}. The syntax should be: Parameter := DataType Identifier", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
        }
    }

    fn fn_statement(&self, pos: usize) -> Result<usize, String> {
        if let Ok(i) = self.fn_assg(pos) { 
            Ok(i)
        } else if let Ok(i) = self.fn_while(pos) {
            Ok(i)
        } else if let Ok(i) = self.fn_if(pos) {
            Ok(i)
        } else if let Ok(i) = self.fn_return(pos) {
            Ok(i)
        } else {
            let i = self.fn_exp(pos)?;
            if self.all_tokens[i].token_type == TokenType::Operator(String::from(";")) {
                return Ok(i + 1);
            }
            Err(format!("Error at Line {} Character {}. The syntax should be: Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)", self.all_tokens[pos].line_num, self.all_tokens[pos].char_pos))
        }
    } 

    fn fn_constant(&self, pos: usize) -> Result<usize, String> {
        let i = pos;
        match &self.all_tokens[i].token_type {
            TokenType::IntConstant(_) => Ok(i+1),
            TokenType::FloatConstant(_) => Ok(i+1),
            _ =>  Err(format!("Error at Line {} Character {}. The syntax should be: Constant := IntContant | FloatContant ", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
        }
    }

    fn fn_datatype(&self, pos: usize) -> Result<usize, String> {
        if let Ok(end) = self.fn_inttype(pos) { // inttype?
            Ok(end)
        } else if let Ok(end) = self.fn_floattype(pos) { //floattype?
            Ok(end)
        } else {
            Err(format!("Error at Line {} Character {}. The syntax should be: DataType := IntegerType | FloatType", self.all_tokens[pos].line_num, self.all_tokens[pos].char_pos))
        }
    }

    fn fn_para_block(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        match &self.all_tokens[i].token_type {
            TokenType::Operator(op) => { // （ ？
                match op.as_str() {
                    "(" => {
                        if self.all_tokens[i+1].token_type == TokenType::Operator(String::from(")")) { // no para just end
                            return Ok(i+2);
                        }
                        i = self.fn_para(i+1)?; // para?
                        loop { // more para?
                            match &self.all_tokens[i].token_type {
                                TokenType::Operator(op) => {
                                    match op.as_str() {
                                        "," => { //check ,
                                            i = self.fn_para(i+1)?;
                                        }
                                        ")" => break, //done with all para
                                        _ => return Err(format!("Error at Line {} Character {}. The syntax should be: ParameterBlock := ( [Parameter {{, Parameter}}] )", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
                                    }
                                }
                                _ => return Err(format!("Error at Line {} Character {}. The syntax should be: ParameterBlock := ( [Parameter {{, Parameter}}] )", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
                            }
                        }
                        Ok(i+1)
                    }
                    _ => Err(format!("Error at Line {} Character {}. The syntax should be: ParameterBlock := ( [Parameter {{, Parameter}}] )", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
                }
            },
            _ => Err(format!("Error at Line {} Character {}. The syntax should be: ParameterBlock := ( [Parameter {{, Parameter}}] )", self.all_tokens[i].line_num, self.all_tokens[i].char_pos)),
        }
    }

    fn fn_block(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        if self.all_tokens[i].token_type == TokenType::Operator(String::from("{")) { 
            i += 1;
            while let Ok(end) = self.fn_dec(i) { //dec?
                i = end;
            }
            while let Ok(end) = self.fn_statement(i) { //state?
                i = end;
            }
            while let Ok(end) = self.fn_fn_def(i) { // fndef?
                i = end;
            }
            if self.all_tokens[i].token_type == TokenType::Operator(String::from("}")) {
                return Ok(i + 1);
            }
        }
        Err(format!("Error at Line {} Character {}. The syntax should be: Block := {{ {{DeclarationType}} {{Statement}} {{FunctionDefinition}} }}", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
    }

    fn fn_fn_dec(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        i = self.fn_para_block(i)?; //parablock?
        if self.all_tokens[i].token_type == TokenType::Operator(String::from(";")) {
            return Ok(i + 1);
        }
        Err(format!("Error at Line {} Character {}. The syntax should be: FunctionDeclaration := ParameterBlock ;", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
    }   

    fn fn_var_dec(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        if self.all_tokens[i].token_type == TokenType::Operator(String::from("=")) { //optional
            i += 1;
            i = self.fn_constant(i)?;
            if self.all_tokens[i].token_type == TokenType::Operator(String::from(";")) {
                return Ok(i + 1);
            }
        } else if self.all_tokens[i].token_type == TokenType::Operator(String::from(";")) { //?
            return Ok(i + 1);
        }
        Err(format!("Error at Line {} Character {}. The syntax should be: VariableDeclaration := [= Constant] ;", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
    }

    fn fn_dec_type(&self, pos: usize) -> Result<usize, String> {
        let i = self.fn_datatype(pos)?; //datatype?
        match &self.all_tokens[i].token_type { //identifier?
            TokenType::Identifier(_) => Ok(i+1), 
            _ => Err(format!("Error at Line {} Character {}. The syntax should be: DeclarationType := DataType Identifier", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
        }
    }

    fn fn_fn_def(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        i = self.fn_dec_type(i)?;
        i = self.fn_para_block(i)?;
        i = self.fn_block(i)?;
        Ok(i)
    }

    fn fn_main(&self, pos: usize) -> Result<usize, String> {
        let mut i = pos;
        
        // println!("{:?}",self.all_tokens[i].token_type);
        if self.all_tokens[i].token_type == TokenType::Keyword(String::from("void")) { //void?
            i += 1;
            // println!("void");
            if self.all_tokens[i].token_type == TokenType::Keyword(String::from("main")) { //main?
                i += 1;
                // println!("main");
                if self.all_tokens[i].token_type == TokenType::Operator(String::from("(")) { //( ?
                    i += 1;
                    // println!("(");
                    if self.all_tokens[i].token_type == TokenType::Operator(String::from(")")) { 
                        i += 1;
                        // println!(")");
                        i = self.fn_block(i)?;
                        return Ok(i);
                    }
                }
            }
        }
        Err(format!("Error at Line {} Character {}. The syntax should be: MainDeclaration := void main() Block", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
    }

    fn fn_dec(&self, pos: usize) -> Result<usize, String> {
        let i = self.fn_dec_type(pos)?; //dec type
        if let Ok(end) = self.fn_var_dec(i) {  //vardec or fndec
            Ok(end)
        } else if let Ok(end) = self.fn_fn_dec(i) {
            Ok(end)
        } else {
            Err(format!("Error at Line {} Character {}. The syntax should be: Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)", self.all_tokens[i].line_num, self.all_tokens[i].char_pos))
        }
    }

    pub fn fn_program(&self) -> Result<(), String> {
        if self.all_tokens.is_empty() { //empty?
            return Err(String::from("program is empty"));
        }
        let mut i = 0; //start at 0
        while let Ok(end) = self.fn_dec(i) { //dec reptition
            i = end;
        }
        i = self.fn_main(i)?; //check main
        while let Ok(end) = self.fn_fn_def(i) { //fn dec reptition
            i = end;
        }
        if self.all_tokens[i].token_type == TokenType::Invalid(String::new()) {
            Ok(())
        } else {
            Err(String::from("parser returned early"))
        }
    }
}

