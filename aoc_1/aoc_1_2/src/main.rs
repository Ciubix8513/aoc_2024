use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");

    let mut first = Vec::new();
    let mut second = Vec::new();

    for i in input.lines() {
        let mut split = i.split("   ");

        first.push(split.next().unwrap().parse::<i32>().unwrap());
        second.push(split.next().unwrap().parse::<i32>().unwrap());
    }
    let mut hash = HashMap::<i32, i32>::new();

    for i in second {
        if let Some(a) = hash.get_mut(&i) {
            *a += 1;
        } else {
            hash.insert(i, 1);
        }
    }

    let o = first
        .iter()
        .fold(0, |o, i| o + i * hash.get(i).unwrap_or(&0));

    println!("{o}");
}
