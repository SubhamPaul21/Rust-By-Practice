// make the necessary variable mutable
fn main() {
    let mut s = String::from("Hello ");
    
    let s1 = &mut s;

    s1.push_str("World!");

    println!("Success!");
}