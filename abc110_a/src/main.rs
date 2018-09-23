use std::cmp::*;
use std::io::*;
use std::str::FromStr;

fn main() {
    let a: u32 = read();
    let b: u32 = read();
    let c: u32 = read();

    let mut v = vec![a, b, c];
    v.sort();

    let sum = v[2] * 10 + v[1] + v[0];
    println!("{}", sum)
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
