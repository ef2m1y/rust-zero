fn main() {
    // function pointer type
    print_it(add, 60, 717);
    print_it(mul, 25, 25);
}

fn print_it(f: fn(u32, u32) -> u32, a: u32, b: u32) {
    println!("{}", f(a, b));
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn mul(a: u32, b: u32) -> u32 {
    a * b
}