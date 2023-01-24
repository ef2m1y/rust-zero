fn main() {
    println!("short-circuit evaluation");
    println!("{}", a() && b());

    println!("non-short-circuit evaluation");
    println!("{}", a() & b());
}

fn a() -> bool {
    println!("call a");
    false
}

fn b() -> bool {
    println!("call b");
    false
}
