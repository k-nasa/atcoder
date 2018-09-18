use std::io::*;
use std::str::FromStr;
use std::vec::*;

fn main() {
    let n = read();

    let mut v: Vec<u32> = Vec::new();

    for i in 0..n {
        let a:u32 = read::<u32>().trailing_zeros();
        v.push(a)
    }

    println!("{}", v.iter().min().unwrap())
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
