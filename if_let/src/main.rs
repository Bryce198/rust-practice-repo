fn main() {
    // This is one way of writing a match statement
    // let config_max = Some(3u8);
    //match config_max {
        //Some(max) => println!("The maximum is configured to be {max}"),
        //_ => (),
    //}

    // The if let syntax allows you to combine if and let keywords into
    // a less verbose way to handle values that match one pattern 

    let config_max = Some(78u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("Number could not be configured")
    }
}
