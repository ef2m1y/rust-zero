fn maybe_fail() -> Option<u32> {
    Some(100)
}
fn main() {
    // shadowing
    let res = maybe_fail();
    {
        let res = res.unwrap();
        println!("{res}");
    }
    println!("{:?}", res);
}
