use std::io::*;
use std::str::FromStr;

fn main() {
    let mut h: i64 = read();

    let mut n = 0;

    h /= 2;

    while(h > 0) {
        n+=1;
        h /= 2;
    }

    let mut result = 0;
    for i in 0..n+1 {
        result += 2i64.pow(i);
    }

    println!("{}", result);
}

/// Example
/// ```
///
///  let n: i32 = read();
///
///  let mut xs = Vec::new();
///  for _ in 0..n {
///      xs.push(read::<i32>() + 1000);
///  }
/// ```
fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}
