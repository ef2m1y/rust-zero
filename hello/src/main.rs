fn main() {
    println!("XOR");
    println!("t^t: {}", xor(true, true));
    println!("t^f: {}", xor(true, false));
    println!("f^t: {}", xor(false, true));
    println!("f^f: {}", xor(false, false));
}

fn xor(x: bool, y: bool) -> bool {
    x ^ y
}