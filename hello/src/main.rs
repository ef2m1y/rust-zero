fn main() {
    // let a: u32;
    // a + 5;
    // -> compile error!

    let n = 5;
    {
        let m = 10;
        let l = n + m;
    }
    // let l = n + m;
    // -> compile error!
}