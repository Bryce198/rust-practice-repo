fn main() {
    let mut v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(8);
    v.push(34);

    println!("Third element of v: {}", &v[2]);
    
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.2),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
