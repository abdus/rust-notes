/**
 * The formula: (32°F − 32) × 5/9
 */
use std::io;

fn main() {
    println!("Press:\n(1) for F to C\n(2) for C to F");

    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Error: failed to read");

    let user_choice: i32 = user_choice
        .trim()
        .parse()
        .expect("Error: failed to parse type String to i32 number");

    if user_choice == 1 {
        println!("{} °C", fahrenheit_to_celsius());
    } else if user_choice == 2 {
        println!("{} °F", celsius_to_fahrenheit())
    } else {
        panic!("Choose either 1 or 2");
    };
}

fn celsius_to_fahrenheit() -> f64 {
    let mut user_input = String::new();
    println!("Please enter a value in celsius");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Panic: could not read value form stdin");

    let user_input: f64 = user_input
        .trim()
        .parse()
        .expect("Error: failed to parse number");

    (user_input * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius() -> f64 {
    let mut user_input = String::new();
    println!("Please enter a value in fahrenheit");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Panic: could not read value form stdin");

    let user_input: f64 = user_input
        .trim()
        .parse()
        .expect("Error: failed to parse number");

    (user_input - 32.0) * 5.0 / 9.0
}
