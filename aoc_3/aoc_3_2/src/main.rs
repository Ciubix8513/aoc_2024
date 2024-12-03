fn main() {
    let input = include_str!("../../input.txt");

    let mut o = 0;
    let mut on = true;

    let mut first = true;

    for i in input.split("don\'t()") {
        if !first {
            on = false;
        }
        first = false;

        let mut first = true;
        for i in i.split("do()") {
            if !first {
                on = true;
            }

            first = false;

            if on {
                for i in i.split("mul") {
                    if i.chars().next().unwrap_or_default() == '('
                        && i.contains(")")
                        && i.contains(",")
                    {
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
            }
        }
    }

    println!("{o}");
}
