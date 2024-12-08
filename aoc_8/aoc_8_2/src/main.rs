use std::collections::HashSet;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coords {
    x: i32,
    y: i32,
}

impl std::ops::Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");

    let width = input.lines().next().unwrap().len() as i32;
    let heght = input.lines().collect::<Vec<_>>().len() as i32;

    let frequencies = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| {
                    (
                        c,
                        Coords {
                            x: x as i32,
                            y: y as i32,
                        },
                    )
                })
        })
        .collect::<Vec<_>>();

    let mut antinodes = HashSet::<Coords>::new();

    for (f, coord) in &frequencies {
        for (f1, coord1) in &frequencies {
            //Ignore if checking the  same coords
            if coord == coord1 {
                continue;
            }

            //Ignore if different frequency
            if f != f1 {
                continue;
            }
            let antinode_dir = *coord1 - *coord;
            let mut antinode_pos = *coord1;
            for _ in 0..50 {
                antinodes.insert(antinode_pos);
                antinode_pos = antinode_pos + antinode_dir;
            }
        }
    }

    let antinodes = antinodes
        .iter()
        .filter(|i| !(i.x < 0 || i.y < 0 || i.x >= width || i.y >= heght))
        .collect::<Vec<_>>();

    println!("{}", antinodes.len())
}
