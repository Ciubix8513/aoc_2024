fn main() {
    let input = include_str!("../../input.txt");

    let mut o = 0;

    for i in input.split("mul") {
        if i.chars().next().unwrap() == '(' && i.contains(")") && i.contains(",") {
            let i = i.trim_matches('(');
            let i = i.split_once(")").unwrap().0;
            if !i.chars().last().unwrap().is_digit(10) || i.contains("(") {
                continue;
            }
            println!("{i}");
            let (num_a, num_b) = i.split_once(',').unwrap();

            o += num_a.parse::<i32>().unwrap() * num_b.parse::<i32>().unwrap();
        }
    }

    println!("{o}");
}
