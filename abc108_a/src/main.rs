use std::io::*;
use std::str::FromStr;
use std::vec::*;

fn main() {
    let k: u32 = read();

    let mut k_count = 0;
    let mut g_count = 0;

    for i in 1..k + 1 {
        if i % 2 == 0 {
            k_count += 1;
        } else {
            g_count += 1;
        }
    }

    println!("{}", k_count * g_count);
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
