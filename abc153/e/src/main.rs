use std::io::*;
use std::str::FromStr;

fn main() {
    let h: isize = read();
    let n: usize = read();

    let mut array = Vec::new();

    for _ in 0..n {
        let a = read::<isize>();
        let b = read::<isize>();

        let e = (a as f64) / (b as f64);

        array.push((e, a, b));
    }

    array.sort_by(|&(a, _, _), &(b, _, _)| b.partial_cmp(&a).unwrap());

    let mut result = 0;
    let mut next_h = h;

    for &(e, a, b) in &array {
        let num = (next_h as isize) / a;

        result += b * num;
        next_h -= a * num;

        println!("{}, {}, {}", next_h, num, result);
    }

    if(next_h > 0) {
        result += array.iter().min_by_key(|b| b.2).unwrap().2;
    }

    println!("{:?}", result);
}

/// Example
/// ```
///
///  let n: i32 = read();
///
///  let mut xs = Vec::new();
///  for _ in 0..n {
///      xs.push(read::<i32>() + 1000);
///  }
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hoge() {
        assert_eq!(run(20, 15), 0);
        assert_eq!(run(20, 30), 0);
        assert_eq!(run(12, 4), 4);
    }
}
