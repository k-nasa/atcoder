use std::io::*;
use std::str::FromStr;

fn main() {
    let mut s_str: String = read();
    let mut t_str: String = read();

    let len = &s_str.len();

    for i in 0..*len {
        if s_str[i..i] == t_str[i..i] {
            s_str[i..i] = t_str[i..i];
        }
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
