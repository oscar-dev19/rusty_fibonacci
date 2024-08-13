use std::io::{self, Write};

fn main() {
    loop {
        let number = input_num_to_calculate();
        match calculate_fib(number) {
            Ok(result) => println!("The fibonacci of given number {} is {}", number, result),
            Err(_) => println!("Number too big! OVERFLOW ERROR!"),
        }

        if !continue_calculation() {
            break;
        }
    }
    println!("Thank you for using the Fibonacci calculator!");
}

fn input_num_to_calculate() -> u128 {
    loop {
        print!("Enter number to calculate fibonacci of: ");
        io::stdout().flush().unwrap();
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read user input!");

        match number.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            },
        };
    }
}

fn calculate_fib(n: u128) -> Result<u128, String> {
    if n <= 1 {
        return Ok(n);
    }

    let mut a = 0u128;
    let mut b = 1u128;

    for _ in 2..=n {
        match b.checked_add(a) {
            Some(sum) => {
                a = b;
                b = sum;
            },
            None => return Err("OVERFLOW".to_string()),
        }
    }

    Ok(b)
}

fn continue_calculation() -> bool {
    loop {
        print!("Do you want to calculate another Fibonacci number? (y/n): ");
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read user input!");

        match response.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Please enter 'y' for yes or 'n' for no.");
                continue;
            }
        }
    }
}
