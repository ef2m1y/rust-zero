fn main() {
    let v = [1, 2, 4, 5, 6, 8, 9, 11];
    let mut sum = 0;
    // v.iter(): 配列の要素への不変参照
    for x in v.iter() {
        if *x % 2 == 0 {
            // continueもラベルを付けることが可能
            continue;
        }
        sum += *x;
    }
    println!("sum of odd numbers: {sum}");

    // for, whileはbreakとともに値を返す事は出来ない
    // -> loopと異なり必ずしもbreakで終了するとは限らないため
}