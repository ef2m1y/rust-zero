fn main() {
    let a: &str = " Hello";
    // a += ", world!";

    let mut b: String = a.to_string();
    b += ", world!  ";
    
    let c: &str = b.trim();
    println!("{c}");
}