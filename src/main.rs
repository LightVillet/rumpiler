mod compiler;

use crate::compiler::compile;

fn main() {
    let code = "int main() { return -!~42; }";
    println!("{}", compile(code.to_string()));
    return ();
}

