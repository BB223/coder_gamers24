use std::collections::HashSet;

fn main() {
    let mut c = 0;
    for n in 2..=100_000_000 {
        let divisors = get_divisors(n);
        let sum: i32 = divisors.iter().sum();
        if sum == n {
            c += 1;
            println!("{}", n);
        }
        if c == 5 {
            break
        }
    }
}

fn get_divisors(n: i32) -> HashSet<i32> {
    let mut res = HashSet::new();
    res.insert(1);
    let sqrt: i32 = f64::ceil(f64::sqrt(n.into())) as i32;
    for test in 2..=sqrt {
        if n % test == 0 {
            res.insert(test);
            res.insert(n/test);
        }
    }
    res
}
