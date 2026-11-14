use std::io;
use std::collections::HashMap;

fn main() {
    println!("Press any key to start program");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read line");

    

    loop {
        display_menu();
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Could not read line");

        let choice: u32 = match user_input.trim().parse() {
            Ok(num) => {
                num
            }
            Err(_) => {
                return
            }
        };
        if choice == 4 {
            println!("Goodbye!");
            break;
        } else if choice == 3 {
            println!("This is option 3");
        }
    }

}

fn display_menu() {
    println!("\nWelcome to Corpo App!");
    println!("\nPick an option down below");
    println!("1) Add Employee");
    println!("2) Delete Employee");
    println!("3) View Employees");
    println!("4) Exit")
}

