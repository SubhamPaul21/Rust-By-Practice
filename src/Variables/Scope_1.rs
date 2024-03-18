// A scope is the range within the program for which the item is valid.

// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    let y: i32 = 10;
    {
        let x = 5;
        let y = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}
