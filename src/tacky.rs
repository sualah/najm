/*
program = Program(function_definition)
function_definition = Function(identifier, instruction* body)
instruction = Return(val) | Unary(unary_operator, val src, val dst)
val = Constant(int) | Var(identifier)
unary_operator = Complement | Negate
 */
use std::fmt;
use std::path::Path;
use crate::lexer::lex;
use crate::parser::{Expr, NParser, Program, Stmt, UnaryOperator};
use crate::tokens::TokenType;
use crate::utils::generate_random_string;

#[derive(Debug, Clone)]
pub struct TackyProgram {
    pub function_definition: Function
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>,
}
#[derive(Debug, Clone)]
pub enum Instruction {
    Return(Val),
    Unary {
        operator: UnaryOperator,
        source: Val,
        destination: Val,
    },
}
#[derive(Debug, Clone)]
pub enum Val {
    Constant(i32),
    Variable(String)
}

// #[derive(Debug, Clone)]
// pub enum UnaryOp {
//     Negate,
//     Complement,
// }


pub struct Tacky {
    pub program: Program,
}


impl Tacky {
    pub fn new(mut parser: NParser) -> Self {
        Self { program :  parser.parse() }
    }
    
    pub fn gen_tacky(&mut self) -> TackyProgram {
        TackyProgram {
            function_definition: self.get_function_definition()
        }
    }

    fn get_function_definition(&mut self) -> Function {

        Function {
            name: self.program.function.name.clone(),
            instructions: self.get_instructions()
        }
    }

    fn get_expression(&mut self, instructions: &mut Vec<Instruction>, expr: Expr) -> Val {
        match expr {
            Expr::Constant(val) => {
                Val::Constant(val)
            },
            Expr::Unary(op, expr) => {
                let src  = self.get_expression(instructions, *expr);
                let dst_name =  generate_random_string(5);
                let dst = Val::Variable(dst_name);

                let una = Instruction::Unary {
                    operator: op.clone(), // use seperate op for tacky and convert Parser op to tacky op
                    source: src.clone(),
                    destination: dst.clone(),
                };
                instructions.push(una);
                dst
            },

        }
    }

    fn get_instructions(&mut self) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        match self.program.function.body.clone() {
            Stmt::Return(expr) => {
               match  self.get_expression(&mut instructions, expr) {  
                   Val::Constant(v) => instructions.push(Instruction::Return(Val::Constant(v))),
                   Val::Variable(var) => instructions.push(Instruction::Return(Val::Variable(var)))
               } ;
           
            }

        };

        instructions
    }
    
}


/*
 pretty printer implementation
 */

impl fmt::Display for TackyProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "TackyProgram {{")?;
        writeln!(f, "  {}", self.function_definition)?;
        writeln!(f, "}}")
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Function {{")?;
        writeln!(f, "    name: \"{}\",", self.name)?;
        writeln!(f, "    instructions: [")?;
        for ins in &self.instructions {
            writeln!(f, "      {},", ins)?;
        }
        writeln!(f, "    ]")?;
        writeln!(f, "  }}")
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Unary { operator, source, destination } => {
                write!(
                    f,
                    "Unary {{ operator: {}, source: {}, destination: {} }}",
                    operator, source, destination
                )
            }
            Instruction::Return(expr) => {
                write!(f, "Return({})", expr)
            }
        }
    }
}



impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Val::Constant(val) => write!(f, "Constant({})", val),
            Val::Variable(name) => write!(f, "Variable(\"{}\")", name),
        }
    }
}