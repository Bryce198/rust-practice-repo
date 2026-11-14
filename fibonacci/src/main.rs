fn main() {
    println!("10th Fibonacci number: {}", fibonacci_sequence(20));
}

fn fibonacci_sequence(number: u32) -> u64 {
    if number == 0 {
        return 0;
    } else {
        let mut a: u64 = 0;
        let mut b: u64 = 1;

        for _ in 2..=number {
            let next = a + b;
            a = b;
            b = next;
            println!("{a}{b}");
        }

        b
    }

}