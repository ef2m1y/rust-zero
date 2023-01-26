fn average(v: &[f32]) -> Option<f32> {
    if v.is_empty() {
        return None;
    }

    let mut sum = 0.0;
    for x in v {
        sum += x;
    }

    Some(sum / v.len() as f32)
}

fn main() {
    let arr = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let res = average(&arr[..]);
    println!("{:?}", res);
}
