use std::thread;

fn process_array(mut starting: Vec<u64>) -> Vec<u64> {
    // let threads = thread::available_parallelism().unwrap_or(NonZero::new(32).unwrap());
    let threads = 64;
    let mut handles = Vec::new();
    for _ in 0..threads.into() {
        let a = starting.split_off(starting.len() / threads);

        handles.push(thread::spawn(move || {
            let mut o = Vec::with_capacity(a.capacity());
            for i in a {
                if i == 0 {
                    o.push(1);
                } else {
                    let digits = i.ilog10() + 1;
                    if digits % 2 == 0 {
                        let splitter = 10u64.pow(digits / 2);

                        let a = i % splitter;
                        let b = (i - a) / splitter;

                        o.push(b);
                        o.push(a);
                    } else {
                        o.push(i * 2024);
                    }
                }
            }
            o
        }));
    }

    let mut o = Vec::new();

    for i in handles {
        o.extend(i.join().unwrap());
    }

    o
}

fn main() {
    let mut array = include_str!("../../input.txt")
        .split_whitespace()
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        array = process_array(array);
    }

    println!("{}", array.len());
}
