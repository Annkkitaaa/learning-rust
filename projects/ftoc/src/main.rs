use std::io;


fn main() {
    println!("enter the temp to convert :");

    let mut f= String::new();
    
    io::stdin()
        .read_line(&mut f)
        .expect("Failed to read line");

    let fahrenheit: f64 = match fahrenheit_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{fahrenheit}°F is equal to {celsius:.2}°C");

    
}
