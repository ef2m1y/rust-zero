fn main() {
    let n: u8 = 0b0001_1000;
    println!("n = {n}"); // 24
    println!("{}", n << 2); // 96 = 0b0110_0000
    println!("{}", n >> 2); // 6 = 0b0000_0110

    let m: i8 = -8;
    println!("m = {m}"); // 0b1111_1000
    println!("{}", m << 2); // -32 = 0b1110_0000
    println!("{}", m >> 2); // -2 = 0b1111_1110
}