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

    loop {
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

                *map.get_mut((position.1) as usize)
                    .unwrap_or(&mut Vec::new())
                    .get_mut((position.0) as usize)
                    .unwrap_or(&mut 0) = -2;
            }
            _ => {}
        }
    }

    let count = map.iter().fold(0, |a, i| {
        a + i.iter().fold(0, |a, i| if *i == -2 { a + 1 } else { a })
    });
    let mut string = String::new();

    for i in map {
        for j in i {
            match j {
                1 => string += "#",
                -1 => string += ".",
                -2 => string += "X",
                _ => {}
            }
        }
        string += "\n";
    }

    println!("{count}");
}
