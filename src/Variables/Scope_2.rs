// Fix the error with the use of define_x
fn main() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> &'static str {
    let x = "hello";
    return x;
}
