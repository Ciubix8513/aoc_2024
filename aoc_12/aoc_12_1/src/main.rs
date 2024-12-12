use std::collections::HashMap;

struct Plot {
    perimiter: usize,
    index: u32,
    plant: char,
}

impl Default for Plot {
    fn default() -> Self {
        Self {
            perimiter: Default::default(),
            index: Default::default(),
            plant: '\n',
        }
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
            let mut sides = 4;
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
                sides -= 1;
            }
            if map
                .get(y)
                .unwrap_or(&Vec::new())
                .get(x + 1)
                .unwrap_or(&'\n')
                == c
            {
                sides -= 1;
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
                sides -= 1;
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
                sides -= 1;
            }
            if id.is_none() {
                id = Some(index);
                index += 1;
            }

            mapped.insert(
                (x, y),
                Plot {
                    index: id.unwrap(),
                    perimiter: sides,
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
        let f = mapped.values().filter(|p| p.index == v);

        let count = f.clone().count();
        if count == 0 {
            continue;
        }
        let perimiter = f.clone().fold(0, |a, i| a + i.perimiter);
        sum += count * perimiter;
    }

    println!("{sum}");
}
