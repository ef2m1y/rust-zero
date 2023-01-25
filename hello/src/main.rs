fn maybe_fail() -> Option<u32> {
    Some(100)
}

fn main() {
    match maybe_fail() {
        // matchは式 -> println!の返り値は()なので返す値の型を合わせる必要
        Some(n) => println!("{n}"),
        None => (),
    }
}
