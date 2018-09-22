use std::io::*;
use std::str::FromStr;

fn main() {
    let n: i32 = read();
    let m: i32 = read();
    let a: i32 = read();
    let b: i32 = read();

    let mut oranges = vec![b; n as usize];

    for _ in 0..m {
        let l: usize = read();
        let r: usize = read();

        for i in l - 1..r {
            oranges[i as usize] = a;
        }
    }

    let sum: i32 = oranges.iter().sum::<i32>();

    println!("{}", sum);
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
