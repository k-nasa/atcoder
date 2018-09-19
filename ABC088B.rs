use std::io::*;
use std::str::FromStr;
use std::vec::*;

fn main() {
    let n: u32 = read();

    let mut a: Vec<u32> = Vec::new();
    for _ in 0..n {
        a.push(read::<u32>());
    }

    a.sort();
    a.reverse();

    let mut a_sum = 0;
    let mut b_sum = 0;

    for i in 0..n {
        if i % 2 == 0 {
            b_sum += a[i as usize];
        } else {
            a_sum += a[i as usize];
        }
    }

    println!("{}", b_sum - a_sum);
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
