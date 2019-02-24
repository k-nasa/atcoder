use std::io::*;
use std::str::FromStr;

fn main() {
    let n: u32 = read();

    let mut input: Vec<f64> = Vec::new();
    let mut pay: Vec<String> = Vec::new();

    for _ in 0..n {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p: Vec<&str> = buf.split_whitespace().collect();

        input.push(p[0].parse().unwrap());
        pay.push(p[1].parse().unwrap());
    }

    let mut sum: f64 = 0.0;

    for i in 0..n {
        match pay[i as usize].as_str() {
            "JPY" => sum += input[i as usize],
            "BTC" => sum += input[i as usize] * 380000.0,
            _ => println!("{}", pay[i as usize]),
        }
    }

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
