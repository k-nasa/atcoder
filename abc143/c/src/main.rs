use std::io::*;
use std::str::FromStr;

fn main() {
    let n: u64 = read();

    let x = Vec::new()
    for _ in 0..n {
        x.push(read::<u64>())
    }

    let result = run(n, &x);

    println!("{}", result);
}

fn run(_n: u64, s: &str) -> usize {
    let mut new = Vec::new();

    let mut before: char = '\0';

    for c in s.chars(){
        if c != before {
            new.push(c)
        }
        before = c;
    }

    new.len()
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
        assert_eq!(run(10, &"aabbbbaaca"), 5);
        assert_eq!(run(5, &"aaaaa"), 1);
        assert_eq!(run(20, &"xxzaffeeeeddfkkkkllq"), 10);
    }
}
