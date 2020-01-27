use std::io::*;
use std::str::FromStr;

fn main() {
    let n: usize = read();
    let k: usize = read();

    let mut hs = Vec::new();
    for _ in 0..n {
        hs.push(read::<usize>());
    }

    // 必殺技を全部使用する

    if hs.len() < k {
        println!("0");
        return;
    }
    hs.sort_by(|a, b| b.cmp(a));

    let hs = &hs[(k as usize)..];

    let result: usize = hs.iter().sum();

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
