use std::io::*;
use std::str::FromStr;

fn main() {
    let n: u64 = read();

    let mut x = Vec::new();
    for _ in 0..n {
        x.push(read::<u64>())
    }

    let result = run(n, &x);

    println!("{}", result);
}

fn run(n: u64, o: &[u64]) -> u64 {
    let mut sum = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            sum += o[i as usize] * o[j as usize]
        }
    }

    sum
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
        assert_eq!(run(3, &[3, 1, 2]), 11);
        assert_eq!(run(7, &[5, 0, 7, 8, 3, 3,2]), 312);
    }
}
