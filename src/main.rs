use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    clear();

    const CHOICES: [&str; 4] = [
        "1 - Convert Temperature",
        "2 - Generate Fibonacci",
        "3 - Twelve Days of XMas",
        "4 - exit",
    ];
    'main: loop {
        clear();
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
        clear();
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
        pause();
    }

    fn celsius_fahrenheit() {
        println!("Enter celsius.");
        let mut celsius: String = String::new();
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");
        let celsius: f64 = match celsius.trim().parse() {
            Ok(celsius) => celsius,
            Err(_) => return println!("Invalid Input.\n"),
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
            Err(_) => return println!("Invalid Input.\n"),
        };
        let celsius: f64 = (fahrenheit - 32.0) / 1.8;
        println!("{fahrenheit} fahrenheit is equal to {celsius} celsius\n");
    }
}

fn nth_fibonacci() {
    let mut fibonacci_map: HashMap<u128, u128> = HashMap::from([(0, 0), (1, 1)]);
    fn fibonacci(n: u128, map: &mut HashMap<u128, u128>) -> u128 {
        match map.get(&n) {
            Some(&v) => return v,
            None => (),
        };
        let v: u128 = fibonacci(n - 1, map) + fibonacci(n - 2, map);
        map.insert(n, v);

        v
    }

    const CHOICES: [&str; 2] = ["1 - find fibonacci.", "4 - return"];
    loop {
        clear();
        println!("nth Fibonacci\n");

        for choice in CHOICES {
            println!("{choice}");
        }

        match get_choice() {
            1 => find_fibonacci(&mut fibonacci_map),
            4 => break,
            _ => continue,
        }
        pause();
    }

    fn find_fibonacci(map: &mut HashMap<u128, u128>) {
        println!("\nEnter a Number.");
        let mut nth: String = String::new();
        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line");
        let nth: u128 = match nth.trim().parse() {
            Ok(nth) => nth,
            Err(_) => return println!("Invalid Input.\n"),
        };

        if nth > 186 {
            return println!("186 is the limit.\n");
        }

        let time: Instant = Instant::now();
        let v: u128 = fibonacci(nth, map);
        let time: Duration = time.elapsed();

        println!("{nth} fibonacci is {v}\n");

        println!("time taken: {:?}", time);
    }
}

fn twelve_days_of_xmas() {
    const DAYS: [(&str, &str, &str); 12] = [
        ("first", "", "A partridge in a pear tree."),
        ("second", "Two", " turtle doves, and"),
        ("third", "Three", " French hens,"),
        ("fourth", "Four", " calling birds,"),
        ("fifth", "Five", " gold rings,"),
        ("sixth", "Six", " geese a-laying,"),
        ("seventh", "Seven", " swans a-swimming,"),
        ("eighth", "Eight", " maids a-milking,"),
        ("ninth", "Nine", " ladies dancing,"),
        ("tenth", "Ten", " lords a-leaping,"),
        ("eleventh", "Eleven", " pipers piping,"),
        ("twelfth", "Twelve", " drummers drumming,"),
    ];
    fn play() {
        let mut day: usize = 0;
        while day < DAYS.len() {
            clear();
            println!("On the {} day of XMas,", DAYS[day].0);
            println!("My true love sent to me");

            let mut rev: usize = day + 1;
            while rev > 0 {
                rev -= 1;
                println!("{}{}", DAYS[rev].1, DAYS[rev].2);
            }
            day += 1;
            pause();
        }
    }

    const CHOICES: [&str; 2] = ["1 - play.", "4 - return"];
    loop {
        clear();
        println!("Twelve Days of XMas\n");

        for choice in CHOICES {
            println!("{choice}");
        }

        match get_choice() {
            1 => play(),
            4 => break,
            _ => continue,
        }
    }
}

// <------------------------------------------> //

fn clear() {
    //! clear the terminal
    os("cls");
}

fn pause() {
    //! pause the terminal
    os("pause");
}

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
