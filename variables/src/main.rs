use std::io;

fn main() {
    // These are expression nunber variables 
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!("5 + 10 = {sum}");
    println!("95.5 - 4.3 = {difference}");
    println!("4 * 30 = {product}");
    println!("56.7 / 32.2 = {quotient}");
    println!("-5 / 3 = {truncated}");
    println!("43 % 5 = {remainder}");
    let sum = sum + product;
    println!("New sum: {sum}");
    
    // Char variables can be declared with single quotes
    // They can also be many other chars than just ASCII.
    let emoji = '😊';
    println!("{emoji}");

    // Strings are groups of chars grouped together.
    let message = "Hello, how are you.";
    println!("{message}");

    // Compound types are used to group multiple values into one
    // type. Rust has two primitve compund types:
    // tuples and arrays.

    // Tuples work about the same as Python or any other lang.
    // They are used to group together different data types in a 
    // fixed length, immutable data struct.

    // Tuples can be deconstructed as shown below using dot notation
    // to access the tuple values 
    // they can also be deconstructed into variables using the method:
    // let (x, y, z) = tupi;
    let tupi = (5_000, 8.8, 1);
    let first = tupi.0;
    let second = tupi.1;
    let third = tupi.2;
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");
    println!("The value of third is: {third}");

    // Array types are another way of collecting mutliple values.
    // Unlike arrays in Python, arrays in rust are of a fixed length.

    // Arrays are declared as such:
    let a = [1, 2, 3, 4, 5]; 

    // Arrays are useful when you want you data allocated on the stack
    // rather than the heap of when you want to ensure that you always have
    // a fixed number of elements.

    // Arrays aren't as flexible as vector types, as vectors are able to 
    // grow and shrink in size.

    // You can access the elements of the array by using indexing as the 
    // following:
    let a_first = a[0];
    let a_second = a[1];
    println!("{a_first}");
    println!("{a_second}");

    // little game:
    println!(" Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

}
