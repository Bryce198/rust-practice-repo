fn main() {
    let number = Some(50);
    plus_one(number);
    plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("There is no value");
            None
        }
        Some(i) => {
            println!(
                "The value of x: {:?}",
                Some(i + 1)
            );
            Some(i + 1)
        }
    }
}