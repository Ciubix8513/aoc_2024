struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

fn main() {
    let mut machines = Vec::new();

    for l in include_str!("../../test_input.txt")
        .lines()
        .collect::<Vec<_>>()
        .chunks(4)
    {
        let a_button = l[0].split_once(':').unwrap().1.split_once(',').unwrap();
        let b_button = l[1].split_once(':').unwrap().1.split_once(',').unwrap();
        let prize = l[2].split_once(':').unwrap().1.split_once(',').unwrap();

        machines.push(Machine {
            a: (
                a_button
                    .0
                    .split_once('X')
                    .unwrap()
                    .1
                    .parse::<u64>()
                    .unwrap(),
                a_button
                    .1
                    .split_once('Y')
                    .unwrap()
                    .1
                    .parse::<u64>()
                    .unwrap(),
            ),
            b: (
                b_button
                    .0
                    .split_once('X')
                    .unwrap()
                    .1
                    .parse::<u64>()
                    .unwrap(),
                b_button
                    .1
                    .split_once('Y')
                    .unwrap()
                    .1
                    .parse::<u64>()
                    .unwrap(),
            ),
            prize: (
                prize.0.split_once('=').unwrap().1.parse::<u64>().unwrap(),
                prize.1.split_once('=').unwrap().1.parse::<u64>().unwrap(),
            ),
        });
    }
    let mut sum = 0;

    for m in machines {
        println!("m");
        let mut options = Vec::new();
        //Each button can only be pressed 100 times
        for x in 0..100 {
            for y in 0..100 {
                let a = (m.a.0 * x, m.a.1 * x);
                let b = (m.b.0 * y, m.b.1 * y);

                if (a.0 + b.0, a.1 + b.1) == m.prize {
                    options.push(x * 3 + y);
                }
            }
        }

        options.sort_unstable();

        if let Some(o) = options.first() {
            sum += o;
        }
    }

    println!("{sum}");
}
