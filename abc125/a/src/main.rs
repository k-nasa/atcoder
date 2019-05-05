fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("{}", run(input))
}

fn run(input: String) -> i32 {
    let v: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (a, b, t) = (v[0], v[1], v[2]);

    let score = t / a * b;
    score
}
