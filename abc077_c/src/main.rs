use std::cmp::Ordering;

fn main() {
    let _n = read_line();

    let mut a = read_numbers();
    let b = read_numbers();
    let mut c = read_numbers();

    a.sort();
    c.sort();

    let mut result = 0;

    let a_len = a.len();

    for b in b {
        let lower = a_len - a[upper_bound(&a, b)..].len();
        let upper = c[lower_bound(&c, b)..].len();

        result += lower * upper;
    }

    println!("{}", result);
}

fn lower_bound(iter: &Vec<usize>, key: usize) -> usize {
    let mut low = 0;
    let mut high = iter.len();

    while low < high {
        let mid = (low + high) / 2;

        match iter[mid].cmp(&key) {
            Ordering::Less | Ordering::Equal => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }

    low
}

fn upper_bound(iter: &Vec<usize>, key: usize) -> usize {
    let mut low = 0;
    let mut high = iter.len();

    while low < high {
        let mid = (low + high) / 2;

        match iter[mid].cmp(&key) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater | Ordering::Equal => high = mid,
        }
    }

    low
}

fn read_line() -> String {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    buf
}

fn read_numbers() -> Vec<usize> {
    read_line()
        .trim()
        .split(" ")
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}
