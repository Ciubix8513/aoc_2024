use std::ops::Mul;

struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

struct Mat {
    m00: f64,
    m01: f64,
    m10: f64,
    m11: f64,
}

impl Mat {
    fn det(&self) -> f64 {
        self.m00 * self.m11 - self.m01 * self.m10
    }
    fn invert(self) -> Option<Self> {
        let det = self.det();
        if det == 0.0 {
            return None;
        }

        let det = 1.0 / det;

        Some(Self {
            m00: self.m11 * det,
            m01: -self.m01 * det,
            m10: -self.m10 * det,
            m11: self.m00 * det,
        })
    }
}

impl Mul<Vec2> for Mat {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.m00 * rhs.x + self.m01 * rhs.y,
            y: self.m10 * rhs.x + self.m11 * rhs.y,
        }
    }
}

#[derive(Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

fn main() {
    let mut machines = Vec::new();

    for l in include_str!("../../input.txt")
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
                prize.0.split_once('=').unwrap().1.parse::<u64>().unwrap() + 10000000000000,
                prize.1.split_once('=').unwrap().1.parse::<u64>().unwrap() + 10000000000000,
            ),
        });
    }
    let mut sum = 0;

    for m in machines {
        let a = Mat {
            m00: m.a.0 as f64,
            m01: m.b.0 as f64,
            m10: m.a.1 as f64,
            m11: m.b.1 as f64,
        };

        let b = Vec2 {
            x: m.prize.0 as f64,
            y: m.prize.1 as f64,
        };

        let x = a.invert().unwrap() * b;

        let int_res_x = x.x.round() as u64;
        let int_res_y = x.y.round() as u64;

        if (x.x - int_res_x as f64).abs() > 0.01 || (x.y - int_res_y as f64).abs() > 0.01 {
            continue;
        }

        sum += int_res_x * 3 + int_res_y;
    }

    println!("{sum}");
}
