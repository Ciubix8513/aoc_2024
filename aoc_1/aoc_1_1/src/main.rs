fn main() {
    let input = include_str!("../../input.txt");

    let mut first = Vec::new();
    let mut second = Vec::new();

    for i in input.lines() {
        let mut split = i.split("   ");

        first.push(split.next().unwrap().parse::<i32>().unwrap());
        second.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    first.sort();
    second.sort();

    let o = first
        .iter()
        .zip(second)
        .fold(0, |o, (a, b)| o + i32::abs(a - b));

    println!("{o}");
}
