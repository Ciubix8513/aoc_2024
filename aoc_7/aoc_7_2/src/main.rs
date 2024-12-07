use std::collections::HashMap;

fn gen_ops(len: u32, ops: Vec<String>) -> Vec<String> {
    let chars = "012";

    if len == 1 {
        return chars.to_string().chars().map(|c| c.to_string()).collect();
    }

    let ops = gen_ops(len - 1, ops);

    let mut new_ops = Vec::new();

    for o in ops {
        for c in chars.chars() {
            new_ops.push(o.clone() + &c.to_string());
        }
    }

    new_ops
}

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

        //No need to regenerate the ops list
        let mut ops_list = HashMap::new();

        for ops in ops_list
            .entry(numbers.len())
            .or_insert(gen_ops(numbers.len() as u32 - 1, Vec::new()))
        {
            let mut r = numbers[0];

            for i in 0..ops.to_string().len() {
                let o = ops.get(i..=i).unwrap();
                match o {
                    "0" => {
                        r *= numbers[i + 1];
                    }
                    "1" => {
                        r += numbers[i + 1];
                    }
                    "2" => {
                        r = (r.to_string() + &numbers[i + 1].to_string())
                            .parse()
                            .unwrap()
                    }
                    _ => {
                        unreachable!()
                    }
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
