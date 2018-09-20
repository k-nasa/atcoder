use std::io::*;
use std::str::FromStr;

fn main() {
    let num: String = read();

    let s = run(num);
    println!("{}", s);
}

fn run(input: String) -> String {
    let mut ans = String::new();

    let num: Vec<i32> = input
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect();

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let a = num[0];
                let b = culc(num[1], i);
                let c = culc(num[2], j);
                let d = culc(num[3], k);

                if 7 == a + b + c + d {
                    ans = change_format(&num, i, j, k);
                }
            }
        }
    }

    ans
}

fn change_format(v: &Vec<i32>, i: u8, j: u8, k: u8) -> String {
    format!(
        "{}{}{}{}{}{}{}=7",
        v[0],
        culc_format(i),
        v[1],
        culc_format(j),
        v[2],
        culc_format(k),
        v[3],
    )
}

fn culc_format(i: u8) -> char {
    if i == 1 {
        '+'
    } else {
        '-'
    }
}

fn culc(a: i32, culc: u8) -> i32 {
    if culc == 1 {
        a
    } else {
        -a
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

#[test]
fn run_test() {
    let tests = vec![("1222", "1+2+2+2=7"), ("0290", "0-2+9+0=7")];
    for (i, (input, output)) in tests.into_iter().enumerate() {
        println!("test:{}", i);
        assert_eq!(run(input.to_string()), output.to_string());
    }
}
