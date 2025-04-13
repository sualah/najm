
use crate::tokens::TokenType;

#[derive(Debug, Clone)]
pub struct Program {
    pub function: FunctionDefinition,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Stmt,
}
#[derive(Debug, Clone)]
pub enum Stmt {
    Return(Expr),
}
#[derive(Debug, Clone)]
pub enum Expr {
    Constant(i32),
    Binary(Box<Expr>, TokenType, Box<Expr>),
}

pub struct NParser {
    tokens: Vec<TokenType>,
    pos: usize,
}

impl NParser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Self {tokens, pos: 0}
    }
    
   pub fn parse(&mut self) -> Program {
        Program {
            function: self.parse_function(),
        }
    }
    fn peek(&self) -> Option<&TokenType> {
        self.tokens.get(self.pos)
    }
    
    fn next(&mut self) -> Option<TokenType> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    
    }

    fn expect(&mut self, expected: &TokenType) {
        let tok = self.next();
        if tok.as_ref() != Some(expected) {
            panic!("Expected {:?} but got {:?}", expected, tok);
        }
    }
    fn parse_function(&mut self) -> FunctionDefinition {
        match self.next() {
            Some(TokenType::Int) => {
                let name = match self.next() {
                    Some(TokenType::Identifier(name)) => name,
                    other => panic!("Expected function name but got {:?}", other),
                };
                self.expect(&TokenType::LeftParen);
              //  self.expect(&TokenType::Void);
                if let Some(TokenType::Void) = self.peek() { //making void optional
                    self.next(); // consume 'void'
                }
                self.expect(&TokenType::RightParen);
                self.expect(&TokenType::LeftBrace);
                let stmt = self.parse_stmt();
                self.expect(&TokenType::RightBrace);
                
                FunctionDefinition {
                    name,
                    body: stmt
                }
            },
            other => panic!("Expected Function but got {:?}", other),
        }
    }
    fn parse_stmt(&mut self) -> Stmt {
        match self.next() {
            Some(TokenType::Return) => {
                let expr = self.parse_expr();
                self.expect(&TokenType::Semicolon);
                Stmt::Return(expr)
            },
           other => panic!("Unexpected token in statement: {:?}", other),
        }
    }
    
    fn parse_expr(&mut self) -> Expr {
     //   let token = take_token(tokens);
        match self.next() {
            Some(TokenType::Constant(val)) => Expr::Constant(val),
            other => panic!("Unexpected token in expression: {:?}", other),
        }
        // let const_value = expect_token(TokenType::Constant(2), tokens);
    }
    
}

// pub fn parse(tokens: &mut Vec<TokenType>) -> Program {
//     println!("tokens: {:?}", tokens);
//     let ast = Program {
//         function: parse_function(tokens),
//     };
//     
//     ast
// }
// 
// fn parse_function(tokens: &mut Vec<TokenType>) -> FunctionDefinition {
//     expect_token(TokenType::Identifier(String::from("int")), tokens);
//     let fn_name = match expect_token(TokenType::Identifier(String::from("main")), tokens) {
//         TokenType::Identifier(value) => value,
//         _ => "".parse().unwrap(),
//     };
//     expect_token(TokenType::LeftParen, tokens);
//     expect_token(TokenType::Identifier(String::from("void")), tokens);
//     expect_token(TokenType::RightParen, tokens);
//     expect_token(TokenType::LeftBrace, tokens);
//     let stmt = parse_stmt(tokens);
//     expect_token(TokenType::RightBrace, tokens);
// 
//     FunctionDefinition {
//         name: fn_name,
//         body: stmt,
//     }
// }
// 
// fn parse_stmt(tokens: &mut Vec<TokenType>) -> Stmt {
//    // expect_token(TokenType::RETURN, tokens);
//     expect_token(TokenType::Identifier(String::from("return")), tokens);
//     let expr = parse_expr(tokens);
//     expect_token(TokenType::Semicolon, tokens);
//     Stmt::Return(expr)
// }
// 
//  fn parse_expr(tokens: &mut Vec<TokenType>) -> Expr {
//      let token = take_token(tokens);
//      match token {
//          TokenType::Constant(val) => Expr::Constant(val),
//          other => {
//              println!("Expected a constant, but got {:?}", other);
//              std::process::exit(1);
//            //  Expr::Constant(const_value)
// 
//          }
//      }
//     // let const_value = expect_token(TokenType::Constant(2), tokens);
//  }
// 
// fn expect_token(expected_token: TokenType, tokens: &mut Vec<TokenType>)  -> TokenType {
//     
//     let actual_token = take_token(tokens);
//     
//     if expected_token != actual_token {
//         println!("expected: {:?} but got: {:?}", expected_token, actual_token);
//         std::process::exit(1);
//     }
//     
//     actual_token
//     
// }
// 
// fn take_token(tokens: &mut Vec<TokenType>) -> TokenType {
//     let token = tokens.remove(0);
//     token
// }

