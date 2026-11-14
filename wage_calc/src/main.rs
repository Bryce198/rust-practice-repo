use std::io;

fn main() {
    println!("Enter your hourly wage:");

    let mut wage = String::new();

    io::stdin()
        .read_line(&mut wage)
        .expect("Could not read the line");

    let wage: i32 = match wage.trim().parse() {
        Ok(num) => {
            num
        }
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    println!("Enter the number of hours your work per week:");

    let mut hours = String::new();

    io::stdin()
        .read_line(&mut hours)
        .expect("Could not read line");

    let hours: i32 = match hours.trim().parse() {
        Ok(num) => {
            num
        }
        Err(_) => {
            return;
        }
    };

    let total = wage * hours;
    

    println!("Your total weekly wage is: {total}");
}
