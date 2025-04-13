use crate::assembly;
use crate::assembly::{Assembly, Instruction, Operand};
use crate::parser::{Program};
use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use anyhow::{Result, Context};


pub fn codegen(program: Program) -> Result<(), anyhow::Error> {
    let mut assembly = Assembly::new(program);
    let assembly_ast = assembly.get_assembly_program();
    println!("assembly program: {:?}", assembly_ast);

    let mut file = OpenOptions::new()
        .write(true)          // overwrite instead of append
        .create(true)
        .truncate(true)
        .open("../../demo.s")
        .context("Failed to open output assembly file")?;

    let mut writer = BufWriter::new(file);


    let fn_def = assembly_ast.function_definition;

    writeln!(writer, "\t.globl\t_{}", fn_def.name)?;
    writeln!(writer, "_{}:", fn_def.name)?;
    
    
    for ins in &fn_def.instructions {
        
        match ins {
            Instruction::Mov { source, destination} => {
            
                let src = match source {
                    Operand::Immediate(value) => format!("${}", value),
                    _ => "".to_string(),
                };
                
                let dst = match destination {
                    Operand::Register(register) => format!("%{}", register),
                    _ => "".to_string(),
                };
                writeln!(writer, "\tmovl\t{}, {}", src, dst)?;
            }
            Instruction::Ret => {
                writeln!(writer, "\tret")?;
            },
        }
    }
    writer.flush().context("Failed to flush output")?;
    Ok(())
}