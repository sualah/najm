use crate::tokens::TokenType;

// #[derive(Debug, Clone)]
// pub enum Function {
//     Function(TokenType, Stmt),
// }
#[derive(Debug, Clone)]
pub struct Program {
    pub function: FunctionDefinition,
   //  Function {
   //  name: String,
   //  body: Stmt
   // },
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
pub fn parse_tokens() -> Program{ 
    // let mut functions: Vec<Function> = Vec::new();
    // 
    // let Function = Function::Function(TokenType::Identifier(String::from("main")), Stmt::Return(Expr::Constant(1)));
    // functions.push(Function);
    // 
    // functions
    let ast = Program {
        function: FunctionDefinition {
            name: "main".to_string(),
            body: Stmt::Return(
                Expr::Constant(42)
            ),
        },
    };
    // let ast = Program::Function {
    //         name: "main".to_string(),
    //         body: Stmt::Return(
    //             Expr::Constant(42)
    //         ),
    // };
    ast
}