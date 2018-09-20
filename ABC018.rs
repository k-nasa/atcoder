use std::io::*;
use std::str::FromStr;

fn main() {
    let mut s: String = read();
    let n = read::<u32>();

    for _ in 0..n {
        let a = read::<usize>();
        let b = read::<usize>();
        let partial = &s.clone()[(a - 1)..b];
        let reverse_partial = &partial.chars().rev().collect::<String>();

        s = s.replace(partial, reverse_partial);
    }

    println!("{}", s);
}

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
