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

fn main() {
    let map_in = include_str!("../../input_map.txt");
    let instr_in = include_str!("../../input_instructions.txt");

    let mut map = Vec::new();
    let mut robot_position = (0, 0);

    for (y, l) in map_in.lines().enumerate() {
        map.push(Vec::new());

        for (x, mut c) in l.chars().enumerate() {
            if c == '@' {
                robot_position = (x, y);
                println!("{robot_position:?}");
                c = '.';
            }

            map[y].push(c);
        }
    }

    for i in instr_in.chars() {
        // print_map(&map, robot_position);
        // println!("");
        // println!("Move {i}");
        match i {
            '>' => match map[robot_position.1][robot_position.0 + 1] {
                '.' => {
                    robot_position.0 += 1;
                    continue;
                }
                '#' => {
                    continue;
                }
                'O' => {
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
                            'O' => {
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
                        map[robot_position.1][robot_position.0 + i] = 'O';
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
                'O' => {
                    let mut can_move = true;
                    let mut ind = 2;
                    loop {
                        match map[robot_position.1 - ind][robot_position.0] {
                            '.' => {
                                //Can move the boxes
                                break;
                            }
                            '#' => {
                                //There's a wall after all the boxes
                                can_move = false;
                                break;
                            }
                            'O' => {
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
                    map[robot_position.1 - 1][robot_position.0] = '.';
                    for i in 2..=ind {
                        map[robot_position.1 - i][robot_position.0] = 'O';
                    }

                    robot_position.1 -= 1;
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
                'O' => {
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
                            'O' => {
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
                        map[robot_position.1][robot_position.0 - i] = 'O';
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
                'O' => {
                    let mut can_move = true;
                    let mut ind = 2;
                    loop {
                        match map[robot_position.1 + ind][robot_position.0] {
                            '.' => {
                                //Can move the boxes
                                break;
                            }
                            '#' => {
                                //There's a wall after all the boxes
                                can_move = false;
                                break;
                            }
                            'O' => {
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
                    map[robot_position.1 + 1][robot_position.0] = '.';
                    for i in 2..=ind {
                        map[robot_position.1 + i][robot_position.0] = 'O';
                    }

                    robot_position.1 += 1;
                }
                _ => unreachable!(),
            },
            '\n' => {}
            _ => unreachable!(),
        }
    }

    // print_map(&map, robot_position);
    let mut gps_sum = 0;
    for (y, i) in map.iter().enumerate() {
        for (x, i) in i.iter().enumerate() {
            if *i == 'O' {
                gps_sum += y * 100 + x;
            }
        }
    }

    println!("{gps_sum}");
}
