fn main() {
    let arr = [20, 50, 23, 5, 7];

    let first_element = arr[1];

    println!("{first_element}");

    for item in arr {
        println!("{}", item + 20);
    }
}
