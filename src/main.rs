mod lexer;
mod parser;
mod codegen;
mod compiler;
mod tokens;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, ExitCode, ExitStatus};
use anyhow::{Context, Result};
use clap::Parser;
use crate::{lexer::lex, parser::NParser};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    codegen: bool,

    path: std::path::PathBuf,
}
fn main()  -> Result<()> {
   // let source_file_name = std::env::args().nth(1).with_context(|| "Missing source file name \nUsage: ./najm <<source-file>>")?;
    let args = Args::parse();

    // let dir = args.path.parent().with_context(|| "failed to get parent dir")?;
    // let output_file = args.path.file_stem().with_context(|| "failed to get file stem")?;
    // 
    //    let preprocess_status = Command::new("gcc")
    //        .arg("-E")
    //        .arg("-P")
    //        .arg(&args.path)
    //         .arg("-o")
    //        .arg(dir.join(output_file).with_extension(".i"))
    //       .status()
    //       .with_context(|| "failed to run gcc")?;
    // 
    //    println!("preprocess_status: gcc exited with status {}", preprocess_status);
    // 
    // 
    // if preprocess_status.success() {
    //     let assembly_status = Command::new("gcc")
    //         .arg("-S")
    //         .arg(dir.join(output_file).with_extension(".i"))
    //         .arg("-o")
    //         .arg(dir.join(output_file).with_extension(".s"))
    //         .status()
    //         .with_context(|| "failed to run gcc")?;  
    //     
    //     println!("assembly_status: gcc exited with status {}", assembly_status);
    //     
    //     if preprocess_status.success() {
    //         let executable_status = Command::new("gcc")
    //             .arg(dir.join(output_file).with_extension(".s"))
    //             .arg("-o")
    //             .arg(dir.join(output_file))
    //             .status()
    //             .with_context(|| "failed to run gcc")?;
    //         println!("executable_status: gcc exited with status {}", executable_status);
    //     }
    // }
    


   

  // let source_file = File::open(&args.path).with_context(|| format!("Could not open file: {}", args.path.display()))?;
  // let reader = BufReader::new(source_file);


    let tokens = lex(&args.path)?;

    if args.lex {
        println!("Lexing file ...");
        println!("{:?}", tokens);
    } else if args.parse {
        let mut parser = NParser::new(tokens);
       // let program = parse(&mut tokens);
        println!("Parsing file: ",);
        println!("{:?}", parser.parse());

    } else if args.codegen {
            println!("Codegen file: {}", args.path.display());
    } else { println!("compiling... {}", args.path.display()) }

    // for line_result in reader.lines() {
    //     let line = line_result.with_context(|| format!("could not read line `{}`", args.path.display()))?;
    //     println!("{}", line);
    // }

    Ok(())
}


