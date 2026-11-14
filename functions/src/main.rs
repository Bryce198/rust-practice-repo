
// In rust, main() is the function that will be called each time
// your program is ran.
fn main() {
    another_function(700);
    print_labled_measurement(5, 'K');
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    
    let z = five();
    println!("The value of z is: {z}");
    
    let t = plus_one(8);
    println!("The value of t is: {t}");

}

// Functions in rust are declared using the fn keyword:
fn another_function(x: u32) {
    println!("The value of x is: {x}");
}

// Functions can take parameters and the param type must be annotated
// like in the function below:
fn print_labled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions in rust return a value by typing the number without
// a semi-colon as the function below does:
fn five() -> i32 {
    5
}

fn plus_one(x: u32) -> u32 {
    x + 1
}