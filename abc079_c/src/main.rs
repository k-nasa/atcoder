use std::io::*;
use std::str::FromStr;
use std::vec::*;

fn main() {
    let num: Vec<i32> = number_to_vec(read::<i32>());

    let mut ans = String::new();

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                if 7 == all_culc(&num, i, j, k) {
                    ans = change_format(&num, i, j, k);
                }
            }
        }
    }

    println!("{}", ans);
}

/// 数字を桁ごとに分割してVecとして返す
/// 今はi32としてもらっているがジェネリクス関数にしたい
fn number_to_vec(i: i32) -> Vec<i32> {
    let s = format!("{0: <04}", i);
    let numbers = s.chars().map(|c| (c as u8 - b'0') as i32);
    numbers.collect()
}

fn change_format(v: &Vec<i32>, i: u8, j: u8, k: u8) -> String {
    let mut s = String::new();
    s.push((v[0] as u8 + b'0') as char);
    s.push(culc_format(i));
    s.push((v[1] as u8 + b'0') as char);
    s.push(culc_format(j));
    s.push((v[2] as u8 + b'0') as char);
    s.push(culc_format(k));
    s.push((v[3] as u8 + b'0') as char);
    s.push('=');
    s.push('7');

    s
}

fn culc_format(i: u8) -> char {
    if i == 1 {
        '+'
    } else {
        '-'
    }
}

// 力技でやっちゃた
fn all_culc(v: &Vec<i32>, i: u8, j: u8, k: u8) -> i32 {
    let a = v[0];
    let b = v[1];
    let c = v[2];
    let d = v[3];

    let a = culc(a, b, i);
    let a = culc(a, c, j);
    let a = culc(a, d, k);

    a
}

fn culc(a: i32, b: i32, culc: u8) -> i32 {
    if culc == 1 {
        a + b
    } else {
        a - b
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
