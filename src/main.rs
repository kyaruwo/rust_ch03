use std::io;

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
            2 => generate_fibonacci(),
            3 => twelve_days_of_xmas(),
            4 => break 'main,
            _ => continue,
        }
    }
}

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

fn generate_fibonacci() {
    println!("Generate Fibonacci\n");
    os("pause");
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
