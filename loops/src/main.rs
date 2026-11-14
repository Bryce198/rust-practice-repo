fn main() {
    //counter();
    //lift_off();
    loop_over_array();
    loop_over_array_2();
    count_down();
    count_up();
}


// Rust has 3 loop types: loop, for loop, and while loop

// The function below is an example of a nested 'loop' type loop:
fn counter() {
    let mut count = 0;
    // The outer loop is labeled with 'counting_up. loops can be accessed by
    // inner loops by calling its label. The label must start with a single quote.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 8 {
                // This break statement breaks the inner loop only,
                // as that is the loop the break is called on by
                // default.
                break;
            }

            if count == 2 {
                // This break statement breaks the outer loop because
                // it called the outer loops 'counting_up label.
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    
    println!("End count = {count}");
}

// While loops loop until the condition stated in the declaration
// turns true. The loop then calls break on itself.
fn lift_off() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFT-OFF!!!");
}

// In this function, the while loop loops over every element in
// the array and breaks when it reaches the maximum number.
fn loop_over_array() {
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", arr[index]);

        index += 1;
    }
}

// You can alternatively and more safely access the elements of an
// array by using a for loop like in the function below. It is more safe
// to access elements in arrs like this because it eliminates the chance 
// of bugs that may occur from going beyond the end of the array.
fn loop_over_array_2() {
    let arr = [1, 2, 3, 4, 5];

    for item in arr {
        println!("the value is: {item}");
    }
}

// using rev (reverse range) to count down from a number
fn count_down() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFT-OFF!!!");
}

fn count_up() {
    for number in (1..4) {
        println!("{number}!");
    }
    println!("LIFT-OFF!!!");
}