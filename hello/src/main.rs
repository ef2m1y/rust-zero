fn main() {
    make_pair::<u8, bool>(10, true);
    make_pair(20, false);
}

fn make_pair<T1, T2>(a: T1, b: T2) -> (T1, T2) {
    (a, b)
}