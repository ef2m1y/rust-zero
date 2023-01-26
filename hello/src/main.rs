fn maybe_fail() -> Option<u32> {
    Some(100)
}

fn use_maybe_fail() -> Option<u32> {
    let n = maybe_fail()?;
    Some(n)
}

fn main() {
    // let n = maybe_fail()?; // error
    println!("{}", use_maybe_fail().unwrap());
}
