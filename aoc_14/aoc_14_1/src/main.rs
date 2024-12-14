struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn main() {
    let input = include_str!("../../input.txt");
    // let dimensions = (11, 7);
    let dimensions = (101, 103);
    let mut robots = Vec::new();

    // p=99,6 v=-44,44
    for r in input.lines() {
        let (p, v) = r.split_once(' ').unwrap();

        let (x, y) = p.split_once('=').unwrap().1.split_once(',').unwrap();
        let (vx, vy) = v.split_once('=').unwrap().1.split_once(',').unwrap();

        robots.push(Robot {
            pos: (x.parse().unwrap(), y.parse().unwrap()),
            vel: (vx.parse().unwrap(), vy.parse().unwrap()),
        });
    }
    let mut quadrant_0 = 0;
    let mut quadrant_1 = 0;
    let mut quadrant_2 = 0;
    let mut quadrant_3 = 0;

    for i in &mut robots {
        i.pos.0 += i.vel.0 * 100;
        i.pos.1 += i.vel.1 * 100;

        i.pos.0 %= dimensions.0;
        i.pos.1 %= dimensions.1;

        i.pos.0 += dimensions.0;
        i.pos.1 += dimensions.1;

        i.pos.0 %= dimensions.0;
        i.pos.1 %= dimensions.1;

        if i.pos.0 < ((dimensions.0 - 1) / 2) {
            if i.pos.1 < ((dimensions.1 - 1) / 2) {
                quadrant_0 += 1;
            } else if i.pos.1 > ((dimensions.1 - 1) / 2) {
                quadrant_2 += 1;
            }
        } else if i.pos.0 > ((dimensions.0 - 1) / 2) {
            if i.pos.1 < ((dimensions.1 - 1) / 2) {
                quadrant_1 += 1;
            } else if i.pos.1 > ((dimensions.1 - 1) / 2) {
                quadrant_3 += 1;
            }
        }
    }

    let safety = quadrant_0 * quadrant_1 * quadrant_2 * quadrant_3;

    println!("{safety}");
}
