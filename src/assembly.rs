
/*
program = Program(function_definition)
function_definition = Function(identifier name, instruction* instructions)
instruction = Mov(operand src, operand dst)
| Unary(unary_operator, operand)
| AllocateStack(int)
| Ret
unary_operator = Neg | Not
operand = Imm(int) | Reg(reg) | Pseudo(identifier) | Stack(int)
reg = AX | R10
 */
use clap::builder::Str;
use crate::assembly::AssemblyInstruction::{Mov, Ret, Unary};
use crate::parser::{Expr, Program, Stmt, UnaryOperator};
use crate::tacky::{TackyProgram, Instruction, Val};

#[derive(Debug, Clone)]
pub struct AssemblyProgram {
    pub function_definition: AssemblyFunction
}

#[derive(Debug, Clone)]
 struct AssemblyFunction {
    pub name: String,
    pub instructions: Vec<AssemblyInstruction>,
}
#[derive(Debug, Clone)]
 pub enum AssemblyInstruction {
    Mov {
        source: Operand,
        destination: Operand,
    },
    Unary {
        op: AssemblyUnaryOp,
        destination: Operand
    },
    AllocateStack(i32),
    Ret
}

#[derive(Debug, Clone)]
pub enum AssemblyUnaryOp {
    Negate,
    Not
}

#[derive(Debug, Clone)]
pub enum Operand {
    Immediate(i32),
    Register(Reg),
    Pseudo(String),
    Stack(i32),
}

#[derive(Debug, Clone)]
enum Reg {
    AX,
    R10,
}



pub struct Assembly {
    pub program: TackyProgram,
}

impl Assembly {
    pub fn new(program: TackyProgram) -> Self {
        Self { program }
    }

    pub fn get_assembly_program(&mut self) -> AssemblyProgram {
        AssemblyProgram {
            function_definition: self.get_function_definition()
        }
    }

    fn get_function_definition(&mut self) -> AssemblyFunction {

        AssemblyFunction {
            name: self.program.function_definition.name.clone(),
            instructions: self.get_instructions()
        }
    }
    fn get_instructions(&mut self) -> Vec<AssemblyInstruction> {
        let mut instructions: Vec<AssemblyInstruction> = Vec::new();
        
        for inst in self.program.function_definition.instructions.clone() {
            match inst {
                Instruction::Return(val) => {
                                let src_operand = match val {
                                    Val::Constant(v) => Operand::Immediate(v),
                                    Val::Variable(id) => Operand::Pseudo(id.clone()),
                                };
                                let mov = Mov {
                                    source: src_operand,
                                    destination: Operand::Register(Reg::AX),
                                };
                                instructions.push(mov);
                                instructions.push(Ret);  
                },
                Instruction::Unary { operator, source, destination} => {
                    let src_operand = match source {
                        Val::Constant(v) => Operand::Immediate(v),
                        Val::Variable(id) => Operand::Pseudo(id.clone())
                    };
                    let dst_operand = match destination {
                        Val::Constant(v) => Operand::Immediate(v),
                        Val::Variable(id) => Operand::Pseudo(id.clone())
                    };
                    let mov = Mov {
                        source: src_operand,
                        destination: dst_operand.clone(),
                    };
                    let unary = Unary {
                        op: match operator {
                            UnaryOperator::Complement => AssemblyUnaryOp::Not,
                            UnaryOperator::Negate => AssemblyUnaryOp::Negate,
                        },
                        destination: dst_operand.clone(),
                    };
                    instructions.push(mov);
                    instructions.push(unary);
                    
                }
            }
        }
       // match self.program.function_definition.instructions.clone() {
            
            // Stmt::Return(expr) => {
            //     match expr { 
            //         Expr::Constant(val) => {
            //             let mov = Mov {
            //                 source: Operand::Immediate(val),
            //                 destination: Operand::Register("eax".to_string()),
            //             };
            //             instructions.push(mov);
            //             instructions.push( Instruction::Ret);  
            //         },
            //        _other => {}
            //     }
            // }

       // };
   
        instructions
    }
}