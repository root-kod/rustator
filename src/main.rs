
use std::io;
fn main() {

    println!("Enter the first number:");
    let mut first_num = String::new();
    io::stdin().read_line(&mut first_num).expect("Failed to read line");

    let first_num: f64 = first_num.trim().parse().expect("Please enter a valid number");

    println!("Enter the operation (+,-,*,/");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();


    println!("Enter the second number:");
    let mut second_num = String::new();
    io::stdin().read_line(&mut second_num).expect("Failed to read line");
    let second_num: f64 = second_num.trim().parse().expect("Please enter a valid number");

    let calc_operation = match operation {
        "+" => Operation::Add(first_num, second_num),
        "-" => Operation::Subtract(first_num, second_num),
        "*" => Operation::Multiply(first_num, second_num),
        "/" => Operation::Divide(first_num, second_num),

        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    let result = calculate(calc_operation);
    println!("Result: {}", result);
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b, 
        Operation::Divide(a, b) => {
            if b == 0.0 {
                panic!("Error: Division by zero!");
            }
            a / b
        }
    }
}