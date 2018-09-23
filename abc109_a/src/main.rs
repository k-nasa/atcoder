use std::io::*;
use std::str::FromStr;

fn main() {
    let a: u8 = read();
    let b: u8 = read();

    if a * b % 2 == 0 {
        println!("No");
    } else {
        println!("Yes");
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
