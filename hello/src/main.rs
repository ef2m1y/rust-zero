fn main() {
    fn sumup(n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            n + sumup(n - 1)
        }
    }
    // if式のボディ部分の型が異なるとcompile error

    println!("{}", sumup(10));
    println!("{}", sumup(100));
}
