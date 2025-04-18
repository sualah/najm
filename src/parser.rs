
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
   // Unary(UnaryOperator, Box<Expr>),
    Binary(Box<Expr>, TokenType, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Complement,
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
        match self.next() {
            Some(TokenType::Constant(val)) => Expr::Constant(val),
            // Some(TokenType::Complement) | Some(TokenType::Negation) => {
            //     let unary_operator = self.parse_unary_op();
            //     let expr = self.parse_expr();
            //     Expr::Unary(unary_operator, Box::from(expr))
            // },
            // Some(TokenType::LeftParen) => {
            //     let expr = self.parse_expr();
            //     self.expect(&TokenType::RightParen);
            //     expr
            // }
            other => panic!("Unexpected token in expression: {:?}", other),
        }
    }
    
    fn parse_unary_op(&mut self) -> UnaryOperator {
        let token = self.peek().unwrap();
        match token  { 
            TokenType::Negation => UnaryOperator::Negate, 
            TokenType::Complement => UnaryOperator::Complement,
            other => panic!("Unexpected token in unary operator: {:?}", other),
        }
        
    }
    
}



