use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    os("cls");

    const CHOICES: [&str; 4] = [
        "1 - Convert Temperature",
        "2 - Generate Fibonacci",
        "3 - Twelve Days of XMas",
        "4 - exit",
    ];
    'main: loop {
        os("cls");

        println!("Chapter 03: Practice\n");

        for choice in CHOICES {
            println!("{choice}");
        }

        match get_choice() {
            1 => convert_temperature(),
            2 => nth_fibonacci(),
            3 => twelve_days_of_xmas(),
            4 => break 'main,
            _ => continue,
        }
    }
}

// <------------------------------------------> //

fn convert_temperature() {
    const CHOICES: [&str; 3] = [
        "1 - Celsius to Fahrenheit",
        "2 - Fahrenheit to Celsius",
        "4 - return",
    ];
    loop {
        os("cls");
        println!("Convert Temperature\n");

        for choice in CHOICES {
            println!("{choice}");
        }

        match get_choice() {
            1 => celsius_fahrenheit(),
            2 => fahrenheit_celsius(),
            4 => break,
            _ => continue,
        }
        os("pause");
    }

    fn celsius_fahrenheit() {
        println!("Enter celsius.");
        let mut celsius: String = String::new();
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");
        let celsius: f64 = match celsius.trim().parse() {
            Ok(celsius) => celsius,
            Err(_) => return,
        };
        let fahrenheit: f64 = (celsius * 1.8) + 32.0;
        println!("{celsius} celsius is equal to {fahrenheit} fahrenheit\n");
    }
    fn fahrenheit_celsius() {
        println!("Enter fahrenheit.");
        let mut fahrenheit: String = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");
        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(fahrenheit) => fahrenheit,
            Err(_) => return,
        };
        let celsius: f64 = (fahrenheit - 32.0) / 1.8;
        println!("{fahrenheit} fahrenheit is equal to {celsius} celsius\n");
    }
}

fn nth_fibonacci() {
    let mut fibonacci_map: HashMap<u128, u128> = HashMap::new();
    fibonacci_map.insert(0, 0);
    fibonacci_map.insert(1, 1);

    const CHOICES: [&str; 2] = ["1 - calculate fibonacci.", "4 - return"];

    loop {
        os("cls");
        println!("nth Fibonacci\n");

        for choice in CHOICES {
            println!("{choice}");
        }

        match get_choice() {
            1 => calculate_fibonacci(&mut fibonacci_map),
            4 => break,
            _ => continue,
        }
        os("pause");
    }

    fn calculate_fibonacci(map: &mut HashMap<u128, u128>) {
        fn fibonacci(n: u128, map: &mut HashMap<u128, u128>) -> u128 {
            let v: u128 = match map.get(&n) {
                Some(&v) => v,
                None => {
                    let v: u128 = fibonacci(n - 1, map) + fibonacci(n - 2, map);
                    map.insert(n, v);
                    v
                }
            };
            v
        }

        println!("\nEnter a Number.");
        let mut nth: String = String::new();
        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line");
        let nth: u128 = match nth.trim().parse() {
            Ok(nth) => nth,
            Err(_) => {
                println!("Not a Number.");
                return os("pause");
            }
        };

        if nth > 186 {
            return println!("186 is my limit uwu.\n");
        }

        let time: Instant = Instant::now();
        let v: u128 = fibonacci(nth, map);
        let time: Duration = time.elapsed();

        println!("{nth} fibonacci is {v}\n");

        println!("time taken: {:?}", time);
    }
}

fn twelve_days_of_xmas() {
    println!("Twelve Days of XMas\n");
    os("pause");
}

// <------------------------------------------> //

fn os(command: &str) {
    //! Execute a Terminal command
    std::process::Command::new("cmd")
        .args(["/C", command])
        .status()
        .expect("Failed to execute {command}");
}

fn get_choice() -> u8 {
    //! Returns choice: u8 else if Err() Returns 0
    println!("\n? - Enter choice.");
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u8 = match choice.trim().parse() {
        Ok(choice) => choice,
        Err(_) => 0,
    };

    choice
}
