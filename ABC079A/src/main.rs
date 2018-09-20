use std::io::*;
use std::str::FromStr;

fn main() {
    let n: String = read();
    let n: Vec<char> = n.chars().collect();

    let mut ans = "No";

    // cloneするのは悪手のような気がする
    for (i, c) in n.clone().into_iter().enumerate() {
        if i >= 2 {
            //2以上の時はいい整数ではない
            break;
        }

        if c == n[i + 1 as usize] {
            if c == n[i + 2 as usize] {
                ans = "Yes";
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
