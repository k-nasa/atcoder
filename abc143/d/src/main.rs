use std::io::*;
use std::str::FromStr;

fn main() {
    let n: usize = read();

    let mut x = Vec::new();
    for _ in 0..n {
        x.push(read::<usize>())
    }

    let result = run(n, &mut x);

    println!("{}", result);
}

fn run(n: usize, x: &mut[usize]) -> usize {
    let mut count = 0;

    x.sort();

    for i in 0..n {
        let a = x[i];
        for j in (i+1)..n {
            let b = x[j];
            for k in (j + 1)..n {
                let c = x[k];

                if (a < b + c) && (b < c + a) && (c < a + b) {
                    count += 1;
                } else {
                    break
                }
            }
        }
    }

    count
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
        assert_eq!(run(4, &mut[3, 4, 2, 1]), 1);
        assert_eq!(run(3, &mut[1, 1000, 1]), 0);
        assert_eq!(run(7, &mut[218,786 ,704, 233, 645, 728, 389]), 23);
    }
}
