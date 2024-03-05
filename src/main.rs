#[macro_use]
mod calculator;
use std::io;

// For next ver I'll modify it in order to build a CLI tool
// TODO: Do the above thing lol
fn main() {
    println!("Macro Calc - Single Calculator macro");

    let mut operator = String::new();
    println!("Type the operator [add/multiply/subtract/divide]: ");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let operator = operator.trim();

    let mut num1 = String::new();
    println!("First number!: ");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Invalid number for num1");

    let mut num2 = String::new();
    println!("Second number!: ");
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Invalid number for num2");

    let result = calculator!(operator, num1, num2);
    println!("Here's ur final result: {}", result);

}
