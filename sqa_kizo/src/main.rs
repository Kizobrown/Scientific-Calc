use std::io;
use std::f64::consts::PI;

fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<f64>().expect("Please enter a number")
}

fn main() {
    loop {
        println!("\n==== Scientific Calculator ====");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Power (x^y)");
        println!("6. Square Root");
        println!("7. Logarithm (base 10)");
        println!("8. Natural Log (ln)");
        println!("9. Trigonometry (sin, cos, tan)");
        println!("0. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap_or(100);

        match choice {
            1 => {
                let a = read_number("Enter first number:");
                let b = read_number("Enter second number:");
                println!("Result = {}", a + b);
            }

            2 => {
                let a = read_number("Enter first number:");
                let b = read_number("Enter second number:");
                println!("Result = {}", a - b);
            }

            3 => {
                let a = read_number("Enter first number:");
                let b = read_number("Enter second number:");
                println!("Result = {}", a * b);
            }

            4 => {
                let a = read_number("Enter numerator:");
                let b = read_number("Enter denominator:");
                if b == 0.0 {
                    println!("Error: Division by zero!");
                } else {
                    println!("Result = {}", a / b);
                }
            }

            5 => {
                let a = read_number("Enter base (x):");
                let b = read_number("Enter exponent (y):");
                println!("Result = {}", a.powf(b));
            }

            6 => {
                let a = read_number("Enter number:");
                if a < 0.0 {
                    println!("Error: Cannot take sqrt of negative number!");
                } else {
                    println!("Result = {}", a.sqrt());
                }
            }

            7 => {
                let a = read_number("Enter number:");
                println!("Result = {}", a.log10());
            }

            8 => {
                let a = read_number("Enter number:");
                println!("Result = {}", a.ln());
            }

            9 => {
                let angle = read_number("Enter angle in degrees:");
                let rad = angle * PI / 180.0;

                println!("sin({}) = {}", angle, rad.sin());
                println!("cos({}) = {}", angle, rad.cos());
                println!("tan({}) = {}", angle, rad.tan());
            }

            0 => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid choice! Please try again."),
        }
    }
}
