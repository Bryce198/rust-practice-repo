fn main() {
    let mut s = String::from("Hello");
    println!("{s}");
    change_string(&mut s);
    println!("{s}");
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", World");
}