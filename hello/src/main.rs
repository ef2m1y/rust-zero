fn main() {
    // ラベルはforやwhileの前にも付けることが可能
    'outer_loop: loop {
        loop {
            break 'outer_loop;
        }
    }
    println!("escaped!");
}