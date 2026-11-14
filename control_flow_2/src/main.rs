use std::io;

fn main() {
    println!("Type in a number:");
    
    let mut config_max = String::new();

    io::stdin()
        .read_line(&mut config_max)
        .expect("Could not read line.");

    let number: u32 = match config_max.trim().parse() {
        Ok(num) => {
            num
        }
        Err(_) => {
            print!("There was an error");
            return;
        }

    };

    match number {
        num => {
            println!(
                "The number is {}",
                num
            );
            println!(
                "{num} + 50 is: {}",
                num + 50
            )
        }
        other => println!("Could not complete function")
    }
    
}
