mod calculator;

use std::io;

fn main() {
    println!("Macro Calc: Single Calculator macro");

    let mut operator = String::new()
    println!("Type the operator [add/multiply/subtract/divide]: ");
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    let operator = operator.trim();


}
