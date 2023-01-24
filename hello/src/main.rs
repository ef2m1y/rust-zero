fn main() {
    let arr: [u32; 4] = [1, 2, 3, 4];
    println!("{}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3]);

    let s: &[u32] = &arr[1..3];
    println!("s = {:?}, s[0] = {}", s, s[0]);

    // arr[10] -> compile error (static analysis)
    // s[10] -> panic (dynamic analysis)

    let arr2: [u32; 5] = [3; 5];
    println!("{:?}", arr2);
}