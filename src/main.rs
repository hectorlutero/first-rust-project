use std::io;
mod operations;

use operations::Operation;
use operations::Addition;
use operations::Subtraction;

fn main() {
    println!("Welcome to the Rust calculator!");
    loop {
        let addition = Addition;
        let subtraction = Subtraction;
        
        fn perform_operation<T: Operation>(operation: T, a: i32, b: i32) -> i32 {
            operation.calculate(a, b)
        }
        println!("Enter your choice: ");
        println!("1. Sum of two values");
        println!("2. Subtraction of two values");
        println!("3. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again.");
                continue;
            },
        };

        if choice == 3 {
            break;
        }

        println!("Enter first value: ");
        let mut first = String::new();
        io::stdin().read_line(&mut first)
            .expect("Failed to read line");
        let first: i32 = match first.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again.");
                continue;
            },
        };

        println!("Enter second value: ");
        let mut second = String::new();
        io::stdin().read_line(&mut second)
            .expect("Failed to read line");
        let second: i32 = match second.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again.");
                continue;
            },
        };


        if choice == 1 {
            println!("The sum of {} and {} is: {}", first, second, perform_operation(addition, first, second));
        } else if choice == 2 {
            println!("The subtraction of {} and {} is: {}", first, second, perform_operation(subtraction, first, second));
        }
    }
}
