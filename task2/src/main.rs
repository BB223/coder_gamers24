use std::{sync::mpsc, thread};

fn main() {
    // let n = 6 - 1;
    // let result = task2(n);
    // println!("n: {} -> {}", n, result);

    let n = 1000 - 1;
    let result = task2(n);
    println!("n: {} -> {}", n, result);

    // let n = 1000 + 1;
    // let result = task2(n);
    // println!("n: {} -> {}", n, result);

    let n = 1000000 - 1;
    let result = task2(n);
    println!("n: {} -> {}", n, result);

    // let n = 1000000 + 1;
    // let result = task2(n);
    // println!("n: {} -> {}", n, result);

    let n = 10000000000000 - 1;
    let result = task2(n);
    println!("n: {} -> {}", n, result);

    // let n = 10000000000000 + 1;
    // let result = task2(n);
    // println!("n: {} -> {}", n, result);
}

fn task2(n: u64) -> u64 {
    let modulo: u64 = 987654321;

    let mut diamond_sum = 0;
    let (tx, rx) = mpsc::channel();

    let mid = n / 2;
    let half = mid / 2;

    let tx_clone = tx.clone();
    thread::spawn(move || {
        println!("1.1");
        let mut part = 0;
        for i in 1..half {
            part += i * n + (mid - i) + 1;
        }
        tx_clone.send(part).unwrap();
    });
    let tx_clone = tx.clone();
    thread::spawn(move || {
        println!("1.2");
        let mut part = 0;
        for i in half..=mid {
            part += i * n + (mid - i) + 1;
        }
        tx_clone.send(part).unwrap();
    });

    // 2. From left to bottom
    let tx_clone = tx.clone();
    thread::spawn(move || {
        println!("2.1");
        let mut part = 0;
        for i in 1..half {
            part += (mid + i) * n + (i - 1) + 1;
        }
        tx_clone.send(part).unwrap();
    });
    let tx_clone = tx.clone();
    thread::spawn(move || {
        println!("2.2");
        let mut part = 0;
        for i in half..=mid {
            part += (mid + i) * n + (i - 1) + 1;
        }
        tx_clone.send(part).unwrap();
    });

    // 3. From bottom to right
    let tx_clone = tx.clone();
    thread::spawn(move || {
        println!("3.1");
        let mut part = 0;
        for i in 1..half {
            part += (n - 1 - i) * n + (mid + i) + 1;
        }
        tx_clone.send(part).unwrap();
    });
    let tx_clone = tx.clone();
    thread::spawn(move || {
        println!("3.2");
        let mut part = 0;
        for i in half..=mid {
            part += (n - 1 - i) * n + (mid + i) + 1;
        }
        tx_clone.send(part).unwrap();
    });

    // 4. From right to top
    let tx_clone = tx.clone();
    thread::spawn(move || {
        println!("4.1");
        let mut part = 0;
        for i in 1..half {
            part += (mid - i) * n + (n - i) + 1;
        }
        tx_clone.send(part).unwrap();
    });
    let tx_clone = tx.clone();
    thread::spawn(move || {
        println!("4.2");
        let mut part = 0;
        for i in half..=mid {
            part += (mid - i) * n + (n - i) + 1;
        }
        tx_clone.send(part).unwrap();
    });

    let mut c = 0;
    for r in rx {
        diamond_sum += r;
        c += 1;
        if c == 8 {
            break;
        }
    }
    diamond_sum % modulo
}
