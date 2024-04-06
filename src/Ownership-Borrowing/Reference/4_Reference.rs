// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;
    
    p.push_str("world");

    println!("Success!");
}