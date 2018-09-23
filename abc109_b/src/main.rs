use std::collections::HashSet;
use std::io::*;
use std::str::FromStr;

fn main() {
    let n: u32 = read();

    let mut w = Vec::new();
    for _ in 0..n {
        w.push(read::<String>());
    }

    let uniq: HashSet<String> = w.clone().into_iter().collect();

    // 同じワードがあるのでNo
    if w.len() != uniq.len() {
        println!("No");
        return;
    }

    for i in 1..n {
        if !w[(i - 1) as usize].ends_with(&w[i as usize][0..1]) {
            println!("No");
            return;
        }
    }

    println!("Yes");
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
