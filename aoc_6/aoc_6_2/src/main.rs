fn main() {
    let input = include_str!("../../input.txt");
    let mut map = Vec::new();

    let mut position = (0, 0);
    let mut direction = (0, -1);

    for (y_index, l) in input.lines().enumerate() {
        map.push(Vec::new());
        for (x_index, c) in l.chars().enumerate() {
            if c == '#' {
                map[y_index].push(1);
            } else {
                if c == '^' {
                    position = (x_index as i32, y_index as i32);
                    map[y_index].push(-2);
                } else {
                    map[y_index].push(-1);
                }
            }
        }
    }
    let starting_position = position;
    let mut count = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut steps = 0;

            //Add new obstacle
            let o = (*map.get_mut(i).unwrap()).get_mut(j).unwrap();

            if *o == 1 {
                continue;
            }
            if (j as i32, i as i32) == starting_position {
                continue;
            }

            *o = 1;

            position = starting_position;
            direction = (0, -1);
            loop {
                if steps >= 30000 {
                    count += 1;
                    break;
                }

                let next_pos = *map
                    .get((position.1 + direction.1) as usize)
                    .unwrap_or(&Vec::new())
                    .get((position.0 + direction.0) as usize)
                    .unwrap_or(&0);
                //try move
                match next_pos {
                    0 => break,
                    1 => match direction {
                        (0, -1) => direction = (1, 0),
                        (1, 0) => direction = (0, 1),
                        (0, 1) => direction = (-1, 0),
                        (-1, 0) => direction = (0, -1),
                        _ => {}
                    },
                    ..=-1 => {
                        position.0 += direction.0;
                        position.1 += direction.1;
                    }
                    _ => {}
                }
                steps += 1;
            }

            // println!("{steps}");
            //Remove it again for the next try
            let o = map.get_mut(i).unwrap().get_mut(j).unwrap();
            *o = -1
        }
    }

    println!("{count}");
}
