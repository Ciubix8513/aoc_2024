use std::collections::HashMap;

fn main() {
    let rules_input = include_str!("../../input.txt");
    let orders = include_str!("../../input_.txt");

    let mut rules = HashMap::<i32, Vec<i32>>::new();

    for r in rules_input.lines() {
        let r = r.split_once('|').unwrap();

        let page = r.0.parse::<i32>().unwrap();
        let before = r.1.parse::<i32>().unwrap();

        rules.entry(page).or_default().push(before);
    }
    let mut sum = 0;

    for o in orders.lines() {
        let mut o = o
            .split(',')
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut valid = true;

        for (index, i) in o.iter().enumerate() {
            if let Some(r) = rules.get(i) {
                for (i_index, item) in o.iter().enumerate() {
                    if r.contains(item) {
                        if i_index < index {
                            valid = false;
                            break;
                        }
                    }
                }
                if !valid {
                    break;
                }
            }
        }

        if valid {
            continue;
        }

        while !valid {
            valid = true;
            let mut swap = None;
            for (index, i) in o.iter().enumerate() {
                if let Some(r) = rules.get(i) {
                    for (i_index, item) in o.iter().enumerate() {
                        if r.contains(item) {
                            if i_index < index {
                                valid = false;
                                swap = Some((i_index, index));
                                break;
                            }
                        }
                    }
                    if !valid {
                        break;
                    }
                }
            }
            if let Some(swap) = swap {
                o.swap(swap.0, swap.1);
            }
        }

        sum += o[o.len() / 2];
    }

    println!("{sum}");
}
