use std::io::*;
use std::str::FromStr;

fn main() {
    let a: i64 = read();
    let b: i64 = read();
    let n: i64 = read();

    let mut a_vec: Vec<i64> = Vec::new();
    let mut b_vec: Vec<i64> = Vec::new();

    for _ in 0..n {
        a_vec.push(read());
        b_vec.push(read());
    }
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
