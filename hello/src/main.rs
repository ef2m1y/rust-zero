fn main() {
    fn sumup_for(n: u64) -> u64 {
        let mut sum: u64 = 0;
        for i in 0..=n {
            sum += i;
        }
        sum
    }

    println!("{}", sumup_for(10));
    println!("{}", sumup_for(100));
}