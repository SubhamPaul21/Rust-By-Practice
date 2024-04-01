// Fix the error with the use of define_x
fn main() {
    let x = define_x1();
    println!("{}, world", x);
}

// Borrowed Value
fn define_x1() -> &'static str {
    let x = "hello";
    return x;
}

// OR

// Owner Value
fn define_x2() -> String {
    let x = "hello".to_string();
    reutrn x;
}
