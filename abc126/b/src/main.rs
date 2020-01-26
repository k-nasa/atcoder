fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let (head, tail) = input.split_at(2);

    let head: i32 = head.parse().unwrap();
    let tail: i32 = tail.trim().parse().unwrap();

    if head > 12 {
        if tail <= 12 && tail > 0 {
            println!("YYMM");
            return;
        } else {
            println!("NA");
            return;
        }
    }

    if head > 0 {
        if tail <= 12 && tail > 0 {
            println!("AMBIGUOUS");
            return;
        }
        if tail > 12 {
            println!("MMYY");
            return;
        }
    }

    if tail <= 12 && tail > 0 {
        println!("YYMM");
        return;
    }
}
