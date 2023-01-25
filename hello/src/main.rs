fn main() {
    fn sumup_loop(mut n: u64) -> u64 {
        let mut sum: u64 = 0;
        loop {
            if n == 0 {
                break sum;
            }
            sum += n;
            n -= 1;
        }
    }

    println!("{}", sumup_loop(10));
    println!("{}", sumup_loop(100));
}
