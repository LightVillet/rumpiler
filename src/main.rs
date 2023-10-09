mod compiler;
use std::env;
use crate::compiler::compile;
use std::fs;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Rumpiler usage: first arg should be a path to your C code file, second arg should be a path to a compiled code.");
        return();
    }
    let in_path = args[1].clone();
    let out_path = args[2].clone();
    println!("Compiling from {in_path} to {out_path}...");
    let code = fs::read_to_string(in_path).unwrap();
    let asm = compile(code);
    let asm_path = format!("{out_path}.s");
    let mut file = std::fs::File::create(asm_path.as_str()).unwrap();
    let _res = file.write_all(asm.as_bytes());
    let mut command = Command::new("gcc").args(["-m32", asm_path.as_str(), "-o", out_path.as_str()]).spawn().unwrap();
    let _result = command.wait().unwrap();
    let _rmv = std::fs::remove_file(asm_path.as_str());
    println!("Done!");
    return();
}

