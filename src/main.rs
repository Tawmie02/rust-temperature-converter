use std::io;

fn main() {
    println!(" Welcome to the Temperature Converter!");

    // Get temperature value
    println!("Enter the temperature value:");
    let mut input_temp = String::new();
    io::stdin().read_line(&mut input_temp).expect("Failed to read input");

    let temp: f64 = match input_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!(" Please enter a valid number.");
            return;
        }
    };

    // Choose conversion direction
    println!("Convert from: (C)elsius or (F)ahrenheit?");
    let mut scale = String::new();
    io::stdin().read_line(&mut scale).expect("Failed to read input");

    let scale = scale.trim().to_uppercase();

    if scale == "C" {
        let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
        println!("{temp}°C is {fahrenheit:.2}°F");
    } else if scale == "F" {
        let celsius = (temp - 32.0) * 5.0 / 9.0;
        println!("{temp}°F is {celsius:.2}°C");
    } else {
        println!(" Invalid choice. Please enter 'C' or 'F'.");
    }
}
