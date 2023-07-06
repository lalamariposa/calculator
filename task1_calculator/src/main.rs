use std::io;

fn main() {
    println!("Enter the first number: ");
    let mut input = String::new();
    io::stdin() //~cin
        .read_line(&mut input) //reads the line
        .expect("Failed to read line"); //it can fail
    let first_num: f64 = input.trim().parse().expect("Input not an integer");
    //trim() - ignore whitesapces around input
    //parse() - convert to integer

    println!("Enter the type of operation you would like to perform (+ , - , * , /): ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation = input.trim().to_string();

    println!("Enter the second number: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_num: f64 = input.trim().parse().expect("Input not an integer"); //Input not an integer

    //println!("First num: {}, Second num: {}, Operation: {}", first_num, second_num, operation);

    let op = match &operation[..] {
        //&str
        "+" => Operation::Add(first_num, second_num),
        "-" => Operation::Subtract(first_num, second_num),
        "*" => Operation::Multiply(first_num, second_num),
        "/" => Operation::Divide(first_num, second_num),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let result: f64 = calculate(op);
    println!("Result: {}", result);
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}
