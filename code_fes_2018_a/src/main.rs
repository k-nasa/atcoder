use std::io::*;
use std::str::FromStr;

fn main() {
    let a: u32 = read();
    let b: u32 = read();
    let c: u32 = read();
    let s: u32 = read();

    let sum = a + b + c;

    if s == sum || s == sum + 1 || s == sum + 2 || s == sum + 3 {
        println!("Yes");
    } else {
        println!("No");
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
