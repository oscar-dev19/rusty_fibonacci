use std::io;

fn main() {
    let number = input_num_to_calculate();
    println!("The fibonacci of given number {} is {}", number, calculate_fib(number));
}

fn input_num_to_calculate() -> i128 {
    loop {
        println!("Enter number to calculate fibonacci of:");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read user input!");

        let number: i64 = match number.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            },
        };
    }
}

fn calculate_fib(num: i128) -> i128 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    for _ in 0..num {
        c = a + b;
        a = b;
        b = c;
    }
    
    c
}
