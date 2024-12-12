use std::collections::HashMap;

struct Plot {
    //0 left, 1 right, 2 up, 3 down
    sides: Vec<char>,
    index: u32,
    plant: char,
}

impl Default for Plot {
    fn default() -> Self {
        Self {
            sides: Default::default(),
            index: Default::default(),
            plant: '\n',
        }
    }
}
fn check_corner(
    sides: &[char],
    d: Option<&[char]>,
    r: Option<&[char]>,
    u: Option<&[char]>,
    l: Option<&[char]>,
) -> usize {
    let mut inner = 0;
    if let Some(l) = l {
        if let Some(d) = d {
            if l.contains(&'d') && d.contains(&'l') {
                inner += 1;
            }
        }
        if let Some(u) = u {
            if l.contains(&'u') && u.contains(&'l') {
                inner += 1;
            }
        }
    }

    if let Some(r) = r {
        if let Some(d) = d {
            if r.contains(&'d') && d.contains(&'r') {
                inner += 1;
            }
        }
        if let Some(u) = u {
            if r.contains(&'u') && u.contains(&'r') {
                inner += 1;
            }
        }
    }

    inner
        + match sides.len() {
            0 | 1 => 0,
            2 => match sides {
                ['r', 'l'] | ['d', 'u'] => 0,
                _ => 1,
            },
            3 => 2,
            4 => 4,
            _ => unreachable!(),
        }
}

fn pass(
    map: &Vec<Vec<char>>,
    mapped: &mut HashMap<(usize, usize), Plot>,
    lp: u32,
    rp: u32,
    up: u32,
    dp: u32,
) {
    loop {
        let mut changes = 0;
        for (y, l) in map.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                let mut id = None;
                for p in 0..4 {
                    //left
                    if lp == p {
                        if map
                            .get(y)
                            .unwrap_or(&Vec::new())
                            .get(x.overflowing_sub(1).0)
                            .unwrap_or(&'\n')
                            == c
                        {
                            if mapped.get(&(x - 1, y)).unwrap_or(&Plot::default()).plant == *c {
                                id = Some(mapped.get(&(x - 1, y)).unwrap().index);
                            }
                        }
                    }
                    if up == p {
                        //up
                        if map
                            .get(y.overflowing_sub(1).0)
                            .unwrap_or(&Vec::new())
                            .get(x)
                            .unwrap_or(&'\n')
                            == c
                        {
                            if mapped.get(&(x, y - 1)).unwrap_or(&Plot::default()).plant == *c {
                                id = Some(mapped.get(&(x, y - 1)).unwrap().index);
                            }
                        }
                    }
                    if rp == p {
                        //right
                        if map
                            .get(y)
                            .unwrap_or(&Vec::new())
                            .get(x + 1)
                            .unwrap_or(&'\n')
                            == c
                        {
                            if mapped.get(&(x + 1, y)).unwrap_or(&Plot::default()).plant == *c {
                                id = Some(mapped.get(&(x + 1, y)).unwrap().index);
                            }
                        }
                    }
                    if dp == p {
                        //down
                        if map
                            .get(y + 1)
                            .unwrap_or(&Vec::new())
                            .get(x)
                            .unwrap_or(&'\n')
                            == c
                        {
                            if mapped.get(&(x, y + 1)).unwrap_or(&Plot::default()).plant == *c {
                                id = Some(mapped.get(&(x, y + 1)).unwrap().index);
                            }
                        }
                    }
                }

                if id.is_none() {
                    //If didn't find an adjacent plant of the same type, skip
                    continue;
                }

                if mapped.get(&(x, y)).unwrap().index == id.unwrap() {
                    continue;
                }

                (mapped.get_mut(&(x, y)).unwrap().index) = id.unwrap();
                changes += 1;
            }
        }

        if changes == 0 {
            break;
        }
    }
}

fn main() {
    let map = include_str!("../../input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // (x,y) , (perimeter, index)
    let mut mapped = HashMap::<(usize, usize), Plot>::new();
    let mut index = 0;

    for (y, l) in map.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            let mut sides = Vec::new();
            let mut id = None;
            //Check sides
            //
            if map
                .get(y + 1)
                .unwrap_or(&Vec::new())
                .get(x)
                .unwrap_or(&'\n')
                == c
            {
                sides.push('d');
            }
            if map
                .get(y)
                .unwrap_or(&Vec::new())
                .get(x + 1)
                .unwrap_or(&'\n')
                == c
            {
                sides.push('r');
            }
            if map
                .get(y.checked_sub(1).unwrap_or(usize::MAX))
                .unwrap_or(&Vec::new())
                .get(x)
                .unwrap_or(&'\n')
                == c
            {
                if mapped.get(&(x, y - 1)).unwrap_or(&Plot::default()).plant == *c {
                    id = Some(mapped.get(&(x, y - 1)).unwrap().index);
                }
                sides.push('u');
            }
            if map
                .get(y)
                .unwrap_or(&Vec::new())
                .get(x.checked_sub(1).unwrap_or(usize::MAX))
                .unwrap_or(&'\n')
                == c
            {
                if mapped.get(&(x - 1, y)).unwrap_or(&Plot::default()).plant == *c {
                    id = Some(mapped.get(&(x - 1, y)).unwrap().index);
                }
                sides.push('l');
            }
            if id.is_none() {
                id = Some(index);
                index += 1;
            }
            let mut new_sides = Vec::new();

            if !sides.contains(&'d') {
                new_sides.push('d');
            }
            if !sides.contains(&'r') {
                new_sides.push('r');
            }
            if !sides.contains(&'u') {
                new_sides.push('u');
            }
            if !sides.contains(&'l') {
                new_sides.push('l');
            }

            mapped.insert(
                (x, y),
                Plot {
                    index: id.unwrap(),
                    sides: new_sides,
                    plant: *c,
                },
            );
        }
    }
    //d l r p
    for rp in 0..4 {
        for lp in 0..4 {
            for up in 0..4 {
                for dp in 0..4 {
                    pass(&map, &mut mapped, lp, rp, up, dp);
                }
            }
        }
    }
    let mut sum = 0;
    for v in 0..index {
        let mut f = mapped.values().filter(|p| p.index == v);

        let count = f.clone().count();
        if count == 0 {
            continue;
        }
        let mut perimiter = 0;
        let current_chunk = mapped
            .iter()
            .filter(|(_, val)| val.index == v)
            .collect::<HashMap<_, _>>();
        for (coord, plot) in &current_chunk {
            perimiter += check_corner(
                &plot.sides,
                current_chunk
                    .get(&(coord.0, coord.1 + 1))
                    .map(|i| i.sides.clone())
                    .as_deref(),
                current_chunk
                    .get(&(coord.0 + 1, coord.1))
                    .map(|i| i.sides.clone())
                    .as_deref(),
                current_chunk
                    .get(&(coord.0, coord.1.overflowing_sub(1).0))
                    .map(|i| i.sides.clone())
                    .as_deref(),
                current_chunk
                    .get(&(coord.0.overflowing_sub(1).0, coord.1))
                    .map(|i| i.sides.clone())
                    .as_deref(),
            )
        }

        // let perimiter = f.clone().fold(0, |a, i| a + check_corner(&i.sides));
        println!("{}, {} * {}", f.next().unwrap().plant, count, perimiter);
        sum += count * perimiter;
    }

    println!("{sum}");
}
