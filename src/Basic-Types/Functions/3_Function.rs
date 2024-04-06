fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
            Some(1)
        }
        _ => {
            // TODO
            None
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    // 1st  Way
    loop {}

    // 2nd way
    panic!("This program will panic!");
}