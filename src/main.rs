use std::io;
use std::io::Write;
fn main() {
    print!("Enter first number: ");
    let mut first_number = String::new();
    io::stdout().flush().unwrap();  
    io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Please type a number!");

    print!("Enter second number: ");
    let mut second_number = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Please type a number!");

    print!("Enter operation (+, -, *, /): ");
    let mut operation = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation: char = operation.trim().parse().expect("Please type a valid operation!");

    let result = match operation {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => panic!("Please type a valid operation (+, -, *, /)")
    };

    println!("Result: {}", result.calculate());
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add(a, b) => a + b,
            Operation::Subtract(a, b) => a - b,
            Operation::Multiply(a, b) => a * b,
            Operation::Divide(a, b) => a / b
        }
    }
}