use std::collections::HashMap;

//Had to look up a solution, since i was completely stuck
//
//Needed to realize that the amount of unique numbers is very small compared to the total amount of
//numbers
fn process_array(input: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut o = HashMap::new();

    for (value, ammount) in input.into_iter() {
        if value == 0 {
            o.entry(1).and_modify(|v| *v += ammount).or_insert(ammount);
        } else {
            let digits = value.ilog10() + 1;
            if digits % 2 == 0 {
                let splitter = 10u64.pow(digits / 2);

                let a = value % splitter;
                let b = (value - a) / splitter;

                o.entry(a).and_modify(|v| *v += ammount).or_insert(ammount);
                o.entry(b).and_modify(|v| *v += ammount).or_insert(ammount);
            } else {
                o.entry(value * 2024)
                    .and_modify(|v| *v += ammount)
                    .or_insert(ammount);
            }
        }
    }
    o
}

fn main() {
    let mut array = include_str!("../../input.txt")
        .split_whitespace()
        .map(|i| (i.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<_, _>>();

    for _ in 0..75 {
        array = process_array(array);
    }

    println!("{}", array.values().sum::<u64>());
    println!("{}", array.len());
}
