
/*
// ASDL for assembly ast
program = Program(function_definition)
function_definition = Function(identifier name, instruction* instructions)
instruction = Mov(operand src, operand dst) | Ret
operand = Imm(int) | Register
 */
use clap::builder::Str;
use crate::assembly::Instruction::Mov;
use crate::parser::{Expr, Program, Stmt};

#[derive(Debug, Clone)]
pub struct AssemblyProgram {
    pub function_definition: Function
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>,
}
#[derive(Debug, Clone)]
pub enum Instruction {
    Mov {
        source: Operand,
        destination: Operand,
    },
    Ret
}

#[derive(Debug, Clone)]
pub enum Operand {
    Immediate(i32),
    Register(String),
}


pub struct Assembly {
    pub program: Program,
}

impl Assembly {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn get_assembly_program(&mut self) -> AssemblyProgram {
        AssemblyProgram {
            function_definition: self.get_function_definition()
        }
    }

    fn get_function_definition(&mut self) -> Function {

        Function {
            name: self.program.function.name.clone(),
            instructions: self.get_instructions()
        }
    }
    fn get_instructions(&mut self) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        match self.program.function.body.clone() {
            Stmt::Return(expr) => {
                match expr { 
                    Expr::Constant(val) => {
                        let mov = Mov {
                            source: Operand::Immediate(val),
                            destination: Operand::Register("eax".to_string()),
                        };
                        instructions.push(mov);
                        instructions.push( Instruction::Ret);  
                    },
                   _other => {}
                }
            }

        };
   
        instructions
    }
}