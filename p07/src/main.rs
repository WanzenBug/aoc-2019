

fn digits(num: usize) -> [usize; 6] {
    [
        num / 100_000 % 10,
        num / 10_000 % 10,
        num / 1_000 % 10,
        num / 100 % 10,
        num / 10 % 10,
        num % 10,
    ]
}

fn ascending_digits(digits: [usize; 6]) -> bool {
    let mut res = true;
    for i in 0..5 {
        res &= digits[i] <= digits[i+1];
    }
    res
}

fn has_adjacent_identical_digits(digits: [usize; 6]) -> bool {
    let mut res = false;
    for i in 0..5 {
        res |= digits[i] == digits[i+1];
    }
    res
}

fn valid(to_check: &usize) -> bool {
    let digits= digits(*to_check);
    ascending_digits(digits) && has_adjacent_identical_digits(digits)
}

fn run(low: usize, high: usize) -> usize {
    (low..=high).filter(valid).count()
}

fn main() {
    eprintln!("run(153517, 630395) = {:#?}", run(153517, 630395));
}
