
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Identifier(String),
    Constant(i32),
    
    //unary
    Negation,
    Complement,
    
    //keywords
    Return,
    Int,
    Void,
    
    
}



