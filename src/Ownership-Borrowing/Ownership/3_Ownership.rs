fn main() {
    let s = give_ownership();
    println!("{:#?}", s);
}

// Only modify the code below!
fn give_ownership() -> Vec<u8> {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.into_bytes().to_vec();
    _s
}