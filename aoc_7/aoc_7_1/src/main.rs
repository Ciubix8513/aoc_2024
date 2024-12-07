fn main() {
    let input = include_str!("../../input.txt");

    let mut sum = 0;

    for l in input.lines() {
        let (a, b) = l.split_once(':').unwrap();

        let res = a.parse::<u64>().unwrap();

        let numbers = b
            .split_whitespace()
            .map(|i| i.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        //Maximum of 16 ops, SHOULD be enough
        for ops in 0..=(0xffff >> (16 - numbers.len() - 1)) {
            let mut r = numbers[0];
            for i in 0..=numbers.len() - 2 {
                if 1 & (ops >> i) == 1 {
                    r *= numbers[i + 1];
                } else {
                    r += numbers[i + 1];
                }
            }

            if r == res {
                sum += res;
                break;
            }
        }
    }

    println!("{sum}");
}
