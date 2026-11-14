fn main() {
    let rectangle_1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rectangle_2 = Rectangle {
        width: 70,
        height: 45
    };
    let rectangle_3 = Rectangle {
        width: 20,
        height: 25
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rectangle_1.area()
    );

    println!(
        "Can rectangle_1 hold rectangle_2? {}",
        rectangle_1.can_hold(&rectangle_2)
    );
    println!(
        "Can rectangle_1 hold rectangle_3? {}",
        rectangle_1.can_hold(&rectangle_3)
    );

    let squ = Rectangle::square(5); // You call associated functions with the <Struct>::<function> syntaxw

    println!(
        "The area of squ: {}",
        squ.area()
    )
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// The impl keyword allows you to create methods and associated
// functions for structs.
impl Rectangle {
    // Methods use &self as the first param in their signature
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height
    }

    // Associated functions do not use &self as a param as they do
    // not rely on an instance of a struct. They can be used to create
    // new instances of the struct just as the function below creates 
    // a new instance of a Rectangle struct.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}