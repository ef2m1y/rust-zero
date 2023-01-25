enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {}

// fn maybe_fail1() -> Option<u32> {
//     // Processing something
//     if success {
//         Some(successful result)
//     } else {
//         None
//     }
// }
//  fn maybe_fail2() -> Result<u32, &str> {
//     // Processing something
//     if success {
//         Ok(successful result)
//     } else {
//         Err("error message")
//     }
//  }