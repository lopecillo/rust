use std::io;

fn tofahrenheit(x: f64) -> f64 {
    x*9.0/5.0+32.0
}

fn tocelsius(x: f64) -> f64 {
    (x-32.0)*5.0/9.0
}

fn main() {
    loop {
        println!("Please enter a temperature in Celsius or Fahrenheit scale. The following examples are valid entries:");
        println!("* 30C, -23.5c for the Celsius scale");
        println!("* 90F, -20f for the Fahrenheit scale");
        println!("* Q, q to quit");

        let mut option = String::new();

        io::stdin()
        .read_line(&mut option)
        .expect("Failed to read option");

        let mut option: String = match option.trim().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };

        // Get the last character (uppercase)
        let scale = option.chars().last().unwrap().to_ascii_uppercase();
        // An empty input will cause unwrap() to panic !!!

        // Remove the last character (the scale)
        option.pop();

        let temperature: f64 = match option.parse::<f64>() {
            Ok(num) => num,
            Err(_) => break,
        };

        match scale {
            'C' => println!("\n{}ºC => {:.1}ºF\n", temperature, tofahrenheit(temperature)),
            'F' => println!("\n{}ºF => {:.1}ºC\n", temperature, tocelsius(temperature)),
            'Q' => break,
            _ => println!("\nUnrecognised. Please enter a valid option\n")
        }
    }
    println!("\nOK, bye!!\n")
}
