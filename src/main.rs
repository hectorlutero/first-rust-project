use std::io;
mod operations;

use operations::Operation;
use operations::Addition;
use operations::Subtraction;
use operations::Multiplication;
use operations::Division;


// Function to display the choices
fn display_choices() {
    println!("Enter your choice: ");
    println!("1. Sum of two values");
    println!("2. Subtraction of two values");
    println!("3. Multiplication of two values");
    println!("4. Division of two values");
    println!("5. Quit");
}

// Function to read and parse the choice input
fn get_choice() -> i32 {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, try again.");
            return get_choice();
        },
    }
}

// Function to read and parse the first value input
fn get_first_value() -> i32 {
    println!("Enter first value: ");
    let mut first = String::new();
    io::stdin().read_line(&mut first)
        .expect("Failed to read line");
    match first.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, try again.");
            return get_first_value();
        },
    }
}

// Function to read and parse the second value input
fn get_second_value() -> i32 {
    println!("Enter second value: ");
    let mut second = String::new();
    io::stdin().read_line(&mut second)
        .expect("Failed to read line");
    match second.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, try again.");
            return get_second_value();
        },
    }
}

fn main() {
    println!("Welcome to the Rust calculator!");
    loop {
        let addition = Addition;
        let subtraction = Subtraction;
        let multiplication = Multiplication;
        let division = Division;
        
        
        fn perform_operation<T: Operation>(operation: T, a: i32, b: i32) -> i32 {
            operation.calculate(a, b)
        }

        display_choices();
        let choice = get_choice();
        if choice == 5 {
            break;
        }
        let first = get_first_value();
        let second = get_second_value();

        if choice == 1 {
            println!("The sum of {} and {} is: {}", first, second, perform_operation(addition, first, second));
        } else if choice == 2 {
            println!("The subtraction of {} and {} is: {}", first, second, perform_operation(subtraction, first, second));
        } else if choice == 3 {
            println!("The multiplication of {} and {} is: {}", first, second, perform_operation(multiplication, first, second));
        } else if choice == 4 {
            println!("The division of {} and {} is: {}", first, second, perform_operation(division, first, second));
        }
    }
}
