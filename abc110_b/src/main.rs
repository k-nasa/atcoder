use std::io::*;
use std::str::FromStr;

fn main() {
    let n: i32 = read();
    let m: i32 = read();
    let x: i32 = 1000 + read::<i32>();
    let y: i32 = 1000 + read::<i32>();

    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for _ in 0..n {
        xs.push(read::<i32>() + 1000);
    }
    for _ in 0..m {
        ys.push(read::<i32>() + 1000);
    }

    let mut ans: String = "War".to_string();
    for i in x + 1..y + 1 {
        if xs.clone().into_iter().max().unwrap() < i {
            if ys.clone().into_iter().min().unwrap() >= i {
                ans = "No War".to_string();
                break;
            }
        }
    }

    println!("{}", ans);
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
