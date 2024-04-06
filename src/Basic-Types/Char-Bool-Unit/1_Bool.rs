// Make println! work
fn main() {
    let _f: bool = false;

    let t = _f;
    if !t {
        println!("Success!");
    }
} 