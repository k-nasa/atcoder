use std::collections::HashMap;
use std::io::*;
use std::option::Option::*;
use std::str::FromStr;

fn main() {
    let n: u32 = read();

    println!("{}", lucas_number(n));
}

fn lucas_number(i: u32) -> u64 {
    let mut hash_map = HashMap::new();
    hash_map.insert(0, 2);
    hash_map.insert(1, 1);

    lucas_calc(i, &mut hash_map)
}

fn lucas_calc(i: u32, map: &mut HashMap<u32, u64>) -> u64 {
    match map.get(&i) {
        Some(&l) => l,
        None => {
            let l = lucas_calc(i - 1, map) + lucas_calc(i - 2, map);
            map.insert(i, l);
            l
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
