use std::collections::HashSet;

struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn print_robots(robots: &[Robot], dim: (i32, i32)) -> String {
    let map = robots.iter().map(|i| i.pos).collect::<HashSet<_>>();
    let mut o = String::new();

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if map.contains(&(x, y)) {
                o += "#";
            } else {
                o += ".";
            }
        }
        o += "\n";
    }

    o
}

fn main() {
    let input = include_str!("../../input.txt");
    let dimensions = (101, 103);
    let mut robots = Vec::new();

    for r in input.lines() {
        let (p, v) = r.split_once(' ').unwrap();

        let (x, y) = p.split_once('=').unwrap().1.split_once(',').unwrap();
        let (vx, vy) = v.split_once('=').unwrap().1.split_once(',').unwrap();

        robots.push(Robot {
            pos: (x.parse().unwrap(), y.parse().unwrap()),
            vel: (vx.parse().unwrap(), vy.parse().unwrap()),
        });
    }

    for i in 0..10000 {
        for i in &mut robots {
            i.pos.0 += i.vel.0;
            i.pos.1 += i.vel.1;

            i.pos.0 %= dimensions.0;
            i.pos.1 %= dimensions.1;

            i.pos.0 += dimensions.0;
            i.pos.1 += dimensions.1;

            i.pos.0 %= dimensions.0;
            i.pos.1 %= dimensions.1;
        }
        let o = print_robots(&robots, dimensions);
        if !o.contains("#####") {
            continue;
        }
        println!("Iteration: {i}");
        println!("{o}");
        println!("");
    }
}
