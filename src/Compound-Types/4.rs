// Fix all errors without adding newline
fn main() {
    let mut s = String::from("hello");
    s += ",";
    s += " world";
    s += "!";

    println!("{}", s);
}