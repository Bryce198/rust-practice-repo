use std::io;

fn main() {
    println!("Enter a temperature in fahrenheit:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let convers = convert_to_celsius(number);

    println!("The tempurature in Celcius is: {convers}");

}

fn convert_to_celsius(temp: i32) -> i32 {
    let conversion: f64 = (temp as f64 - 32.0) / 1.8;
    conversion as i32
}
