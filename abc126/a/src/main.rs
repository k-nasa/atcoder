fn main() {
    let mut k = String::new();
    std::io::stdin().read_line(&mut k).unwrap();
    let v: Vec<u32> = k.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    run(s, v[1] - 1)
}

fn run(s: String, i: u32) {
    let mut chars: Vec<char> = s.chars().collect();

    chars[i as usize] = (chars[i as usize] as u8 + 0x20) as char;

    for c in chars {
        print!("{}", c)
    }
}
