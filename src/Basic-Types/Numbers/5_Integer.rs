// Fix errors and panics to make it work
fn main() {
    let v1: u8 = 247_u16 + 8;
    let v2: i8 = i8::checked_add(119, 8).unwrap();
    println!("{},{}", v1, v2);
}
