fn main() {
    let number = 9;

    if number < 7{
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number_2 = 0;

    if number_2 != 0 {
        println!("The number was something other than zero");
    } else {
        println!("The number is equal to zero");
    }

    let number_3 = 6;

    if number_3 % 4 == 0 {
        println!("{number_3} is divisible by 4");
    } else if number_3 % 3 == 0 {
        println!("{number_3} is divisible by 3");
    } else if number_3 % 2 == 0 {
        println!("{number_3} is divisible by 2");
    } else {
        println!("{number_3} is not divisible by 4, 3, or 2.")
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of the number is: {number}");
}
