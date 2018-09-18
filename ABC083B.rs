use std::io::*;
use std::str::FromStr;

fn main() {
    let n: u32 = read();
    let a: u32 = read();
    let b: u32 = read();

    let mut count = 0;

    for i in 1..n + 1 {
        let numStr = i.to_string();

        let sum = numStr
            .chars()
            .map(|num| (num as u8 - b'0') as u32)
            .sum::<u32>();
        if a <= sum && sum <= b {
            count += i;
        };
    }

    println!("{}", count);
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
