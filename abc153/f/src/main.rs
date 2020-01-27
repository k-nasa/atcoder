use std::io::*;
use std::str::FromStr;

fn main() {
    let a: i64 = read();
    let b: i64 = read();

    let result = run(a, b);

    println!("{}", result);
}

fn run(a: i64, b: i64) -> i64 {
    let x = a - (2 * b);

    if x > 0 {
        x
    } else {
        0
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hoge() {
        assert_eq!(run(20, 15), 0);
        assert_eq!(run(20, 30), 0);
        assert_eq!(run(12, 4), 4);
    }
}
