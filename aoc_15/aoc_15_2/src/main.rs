fn print_map(map: &[Vec<char>], r_pos: (usize, usize)) {
    for (y, i) in map.iter().enumerate() {
        for (x, i) in i.iter().enumerate() {
            if (x, y) == r_pos {
                print!("@");
                continue;
            }
            print!("{i}");
        }
        print!("\n")
    }
}

fn try_move_boxes_down(map: &Vec<Vec<char>>, mut pos: (usize, usize)) -> bool {
    //Make so that it's always pushing from the right
    if map[pos.1][pos.0] == '[' {
        pos.0 += 1
    }
    //If found a wall, can't move
    if map[pos.1 + 1][pos.0] == '#' || map[pos.1 + 1][pos.0 - 1] == '#' {
        return false;
    }

    //If empty space, can move
    if map[pos.1 + 1][pos.0] == '.' && map[pos.1 + 1][pos.0 - 1] == '.' {
        return true;
    }

    let mut a = true;
    let mut b = true;

    if map[pos.1 + 1][pos.0] == '[' || map[pos.1 + 1][pos.0] == ']' {
        a = try_move_boxes_down(&map, (pos.0, pos.1 + 1));
    }
    if map[pos.1 + 1][pos.0 - 1] == '[' || map[pos.1 + 1][pos.0 - 1] == ']' {
        b = try_move_boxes_down(&map, (pos.0 - 1, pos.1 + 1));
    }

    a && b
}

fn move_boxes_down(map: &mut Vec<Vec<char>>, mut pos: (usize, usize)) {
    //Make so that it's always pushing from the right
    if map[pos.1][pos.0] == '[' {
        pos.0 += 1
    }
    if map[pos.1 + 1][pos.0] == '[' || map[pos.1 + 1][pos.0] == ']' {
        move_boxes_down(map, (pos.0, pos.1 + 1));
    }
    if map[pos.1 + 1][pos.0 - 1] == '[' || map[pos.1 + 1][pos.0 - 1] == ']' {
        move_boxes_down(map, (pos.0 - 1, pos.1 + 1));
    }

    map[pos.1 + 1][pos.0] = ']';
    map[pos.1 + 1][pos.0 - 1] = '[';
    map[pos.1][pos.0] = '.';
    map[pos.1][pos.0 - 1] = '.';
}

fn try_move_boxes_up(map: &Vec<Vec<char>>, mut pos: (usize, usize)) -> bool {
    //Make so that it's always pushing from the right
    if map[pos.1][pos.0] == '[' {
        pos.0 += 1
    }
    //If found a wall, can't move
    if map[pos.1 - 1][pos.0] == '#' || map[pos.1 - 1][pos.0 - 1] == '#' {
        return false;
    }

    //If empty space, can move
    if map[pos.1 - 1][pos.0] == '.' && map[pos.1 - 1][pos.0 - 1] == '.' {
        return true;
    }

    let mut a = true;
    let mut b = true;

    if map[pos.1 - 1][pos.0] == '[' || map[pos.1 - 1][pos.0] == ']' {
        a = try_move_boxes_up(&map, (pos.0, pos.1 - 1));
    }
    if map[pos.1 - 1][pos.0 - 1] == '[' || map[pos.1 - 1][pos.0 - 1] == ']' {
        b = try_move_boxes_up(&map, (pos.0 - 1, pos.1 - 1));
    }

    a && b
}

fn move_boxes_up(map: &mut Vec<Vec<char>>, mut pos: (usize, usize)) {
    //Make so that it's always pushing from the right
    if map[pos.1][pos.0] == '[' {
        pos.0 += 1
    }
    if map[pos.1 - 1][pos.0] == '[' || map[pos.1 - 1][pos.0] == ']' {
        move_boxes_up(map, (pos.0, pos.1 - 1));
    }
    if map[pos.1 - 1][pos.0 - 1] == '[' || map[pos.1 - 1][pos.0 - 1] == ']' {
        move_boxes_up(map, (pos.0 - 1, pos.1 - 1));
    }

    map[pos.1 - 1][pos.0] = ']';
    map[pos.1 - 1][pos.0 - 1] = '[';
    map[pos.1][pos.0] = '.';
    map[pos.1][pos.0 - 1] = '.';
}

fn main() {
    let map_in = include_str!("../../input_map.txt");
    let instr_in = include_str!("../../input_instructions.txt");
    // let map_in = include_str!("../../test_input_map_1.txt");
    // let instr_in = include_str!("../../test_input_instructions_1.txt");

    let mut map = Vec::new();
    let mut robot_position = (0, 0);

    for (y, l) in map_in.lines().enumerate() {
        map.push(Vec::new());

        for (x, c) in l.chars().enumerate() {
            if c == '@' {
                robot_position = (x * 2, y);
                map[y].push('.');
                map[y].push('.');
            }
            if c == '#' {
                map[y].push('#');
                map[y].push('#');
            }
            if c == 'O' {
                map[y].push('[');
                map[y].push(']');
            }
            if c == '.' {
                map[y].push('.');
                map[y].push('.');
            }
        }
    }

    for i in instr_in.chars() {
        if i == '\n' {
            continue;
        }
        // println!("Move {i}");
        // print_map(&map, robot_position);
        // println!("");
        match i {
            '>' => match map[robot_position.1][robot_position.0 + 1] {
                '.' => {
                    robot_position.0 += 1;
                    continue;
                }
                '#' => {
                    continue;
                }
                '[' => {
                    let mut can_move = true;
                    let mut ind = 2;
                    loop {
                        match map[robot_position.1][robot_position.0 + ind] {
                            '.' => {
                                //Can move the boxes
                                break;
                            }
                            '#' => {
                                //There's a wall after all the boxes
                                can_move = false;
                                break;
                            }
                            '[' | ']' => {
                                ind += 1;
                                continue;
                            }
                            _ => unreachable!(),
                        }
                    }
                    if !can_move {
                        continue;
                    }
                    //Move boxes...
                    map[robot_position.1][robot_position.0 + 1] = '.';
                    for i in 2..=ind {
                        map[robot_position.1][robot_position.0 + i] = '[';
                        if map[robot_position.1][robot_position.0 + i - 1] == '[' {
                            map[robot_position.1][robot_position.0 + i] = ']';
                        }
                    }

                    robot_position.0 += 1;
                }
                _ => unreachable!(),
            },
            '^' => match map[robot_position.1 - 1][robot_position.0] {
                '.' => {
                    robot_position.1 -= 1;
                    continue;
                }
                '#' => {
                    continue;
                }
                '[' | ']' => {
                    if try_move_boxes_up(&map, (robot_position.0, robot_position.1 - 1)) {
                        robot_position.1 -= 1;
                        move_boxes_up(&mut map, robot_position);
                    }
                    // else {
                    //     println!("Can't move up");
                    // }
                }
                _ => unreachable!(),
            },
            '<' => match map[robot_position.1][robot_position.0 - 1] {
                '.' => {
                    robot_position.0 -= 1;
                    continue;
                }
                '#' => {
                    continue;
                }
                ']' => {
                    let mut can_move = true;
                    let mut ind = 2;
                    loop {
                        match map[robot_position.1][robot_position.0 - ind] {
                            '.' => {
                                //Can move the boxes
                                break;
                            }
                            '#' => {
                                //There's a wall after all the boxes
                                can_move = false;
                                break;
                            }
                            '[' | ']' => {
                                ind += 1;
                                continue;
                            }
                            _ => unreachable!(),
                        }
                    }
                    if !can_move {
                        continue;
                    }
                    //Move boxes...
                    map[robot_position.1][robot_position.0 - 1] = '.';
                    for i in 2..=ind {
                        map[robot_position.1][robot_position.0 - i] = ']';
                        if map[robot_position.1][robot_position.0 - i + 1] == ']' {
                            map[robot_position.1][robot_position.0 - i] = '[';
                        }
                    }

                    robot_position.0 -= 1;
                }
                _ => unreachable!(),
            },
            'v' => match map[robot_position.1 + 1][robot_position.0] {
                '.' => {
                    robot_position.1 += 1;
                    continue;
                }
                '#' => {
                    continue;
                }
                '[' | ']' => {
                    if try_move_boxes_down(&map, (robot_position.0, robot_position.1 + 1)) {
                        robot_position.1 += 1;
                        move_boxes_down(&mut map, robot_position);
                    }
                }
                _ => unreachable!(),
            },
            '\n' => {}
            _ => {
                println!("{i}");
                unreachable!()
            }
        }
    }

    print_map(&map, robot_position);
    let mut gps_sum = 0;
    for (y, i) in map.iter().enumerate() {
        for (x, i) in i.iter().enumerate() {
            if *i == '[' {
                gps_sum += y * 100 + x;
            }
        }
    }

    println!("{gps_sum}");
}
