fn maybe_fail() -> Option<u32> {
    Some(100)
}

fn main() {
    if let Some(n) = maybe_fail() {
        println!("{n}");
    }

    while let Some(n) = maybe_fail() {
        println!("{n}")
    }
}
