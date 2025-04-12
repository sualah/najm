
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Identifier(String),
    Constant(i32),
    
    //keywords
    Return,
    Int,
    Void,
    
}



