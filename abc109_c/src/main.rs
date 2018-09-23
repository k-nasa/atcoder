use std::cmp::*;
use std::io::*;
use std::str::FromStr;
use std::vec::*;

fn main() {
    let n: u64 = read();
    let x: u64 = read();

    let mut abs_vec: Vec<i64> = Vec::new();

    for _ in 0..n {
        let xi: i64 = read();
        abs_vec.push((xi - x as i64).abs());
    }

    let mut ans = abs_vec[0];

    for i in abs_vec {
        ans = gcd(max(ans, i), min(ans, i));
    }

    println!("{}", ans);
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
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
