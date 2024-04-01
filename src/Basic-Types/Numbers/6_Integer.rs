// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    /*
    0xff -->
        * the prefix 0x indicates a hexadecimal literal. So 0xff is a way to represent the hexadecimal number FF, which is equivalent to the decimal number 255.
        F * 16^1 + F * 16^0
        = 15 * 16 + 15 * 1
        = 240 + 15
        = 255
     */

    /*
    0o77 -->
        *  the prefix 0o indicates an octal literal. So 0o77 is a way to represent the octal number 77, which is equivalent to the decimal number 63.
        7 * 8^1 + 7 * 8^0
        = 7 * 8 + 7 * 1
        = 56 + 7
        = 63
     */

    /*
    0b1111_1111
        * the prefix 0b indicates a binary literal. So 0b1111_1111 is a way to represent the binary number 1111_1111, which is equivalent to the decimal number 255.
        1 * 2^7 + 1 * 2^6 + 1 * 2^5 + 1 * 2^4 + 1 * 2^3 + 1 * 2^2 + 1 * 2^1 + 1 * 2^0
        = 128 + 64 + 32 + 16 + 8 + 4 + 2 + 1
        = 255
     */

    assert!(v == 1597);
    println!("Success!");
}
