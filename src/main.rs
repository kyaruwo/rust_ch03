use std::io;
use std::process::Command;

fn sys(command: &str) {
    Command::new("cmd")
        .args(["/C", command])
        .status()
        .expect("Failed to execute {command}");
}

fn main() {
    sys("cls");
    'main: loop {
        sys("cls");

        println!("Chapter 03: Practice\n");

        const CHOICES: [&str; 4] = [
            "a - Convert Temperature",
            "b - Generate Fibonacci",
            "c - Twelve Days of XMas",
            "x - exit",
        ];
        for choice in CHOICES {
            println!("{choice}");
        }
        println!("\n? - Enter choice below");

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: char = match choice.trim().parse() {
            Ok(choice) => choice,
            Err(_) => continue,
        };

        match choice {
            'a' => convert_temperature(),
            'b' => generate_fibonacci(),
            'c' => twelve_days_of_xmas(),
            'x' => break 'main,
            _ => continue,
        }
        sys("pause");
    }
}

fn convert_temperature() {
    println!("\nConvert Temperature\n");
}

fn generate_fibonacci() {
    println!("\nGenerate Fibonacci\n");
}

fn twelve_days_of_xmas() {
    println!("\nTwelve Days of XMas\n");
}
