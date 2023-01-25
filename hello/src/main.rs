fn main() {
    fn sumup_while(mut n: u64) -> u64 {
        let mut sum: u64 = 0;
        while n > 0 {
            sum += n;
            n -= 1;
        }
        sum
    }

    println!("{}", sumup_while(10));
    println!("{}", sumup_while(100));
}