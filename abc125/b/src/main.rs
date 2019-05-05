fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();

    let mut v = String::new();
    std::io::stdin().read_line(&mut v).unwrap();
    let v: Vec<i32> = v.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut c = String::new();
    std::io::stdin().read_line(&mut c).unwrap();
    let c: Vec<i32> = c.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut score = 0;
    for (v, c) in v.iter().zip(c.iter()) {
        if v > c {
            score += v - c;
        }
    }

    println!("{}", score)
}
