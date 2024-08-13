# 🌀 Fibonacci Calculator in Rust

Welcome to the **Fibonacci Calculator**, a Rust-powered program that computes Fibonacci numbers with blazing speed and efficiency! Whether you're a math enthusiast or just looking to explore the magical world of Fibonacci sequences, this program is here to help.

## 🚀 Features

- **Interactive Interface**: Enter a number and watch the Fibonacci magic happen right before your eyes!
- **Overflow Management**: Handles large numbers with care, preventing those pesky overflow errors.
- **Continue or Exit**: Flexibility to calculate as many Fibonacci numbers as you want or gracefully exit whenever you're done.

## 🛠️ How It Works

1. **Input Your Number**: The program prompts you to enter a positive integer.
2. **Calculation**: The Fibonacci number corresponding to your input is calculated with precision.
3. **Result Display**: The result is displayed instantly, unless the number is too large—then, you'll get a friendly warning.
4. **Repeat or Exit**: You can choose to calculate another Fibonacci number or exit the program.

## 🧩 Example

```bash
Enter the number to calculate its Fibonacci value: 10
The Fibonacci of 10 is 55

Would you like to calculate another number? (y/n): y
Enter the number to calculate its Fibonacci value: 1000
Number too big! OVERFLOW ERROR!

Would you like to calculate another number? (y/n): n
Thank you for using the Fibonacci calculator!
```
💻 Installation & Usage
---
0. Install rust
   I recommend using [rustup](https://rustup.rs/)
2. Clone the Repository:
   ```bash
   git clone https://github.com/oscar-dev19/fibonacci-calculator.git
   cd fibonacci-calculator
   ```
3. Compile the program.
   ```bash
   cargo build --release
   ```
4. Run the Program:
   ```bash
   ./target/release/fibonacci-calculator
    ```

🧠 Behind the Scenes
The Fibonacci sequence is calculated using a recursive or iterative method, depending on the size of the input. Rust's u128 is used to handle large numbers, but even this has its limits—when you hit that limit, the program lets you know in a friendly way.

🛡️ Error Handling
Overflow: If the number is too large for the calculation, the program will display an "OVERFLOW ERROR!" message.
Invalid Input: The program ensures only valid positive integers are accepted for calculation.
🙌 Contributing
Contributions, issues, and feature requests are welcome! Feel free to check out the issues page if you want to contribute.

📜 License
This project is licensed under the MIT License. See the [LICENSE](https://opensource.org/license/mit) file for details.
