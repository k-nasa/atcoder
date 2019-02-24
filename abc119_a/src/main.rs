use std::io::*;
use std::str::FromStr;

fn main() {
    let input: String = read();
    let input: Vec<&str> = input.split('/').collect();

    let year: u32 = input.first().unwrap().parse().unwrap();
    let month: u32 = input[1].parse().unwrap();

    if year < 2019 {
        return println!("Heisei");
    }

    if month <= 4 {
        return println!("Heisei");
    }
    return println!("TBD");
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
