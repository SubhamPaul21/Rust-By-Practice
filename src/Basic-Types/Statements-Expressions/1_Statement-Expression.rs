// Make it work with two ways

// 1 esy
fn main() {
    let v = {
        let mut x = 1;
        x + 2
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
}

// 2nd way
// Make it work with two ways
fn main() {
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, ());
 
    println!("Success!");
}