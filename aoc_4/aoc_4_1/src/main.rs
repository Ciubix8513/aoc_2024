fn main() {
    let input = include_str!("../../input.txt");

    let lines = input.lines().collect::<Vec<_>>();

    let mut counter = 0;

    for (index, l) in lines.iter().enumerate() {
        for (c_index, c) in l.chars().enumerate() {
            //Found X
            if c == 'X' {
                //Search forward
                if l.get(c_index..c_index + 4).unwrap_or_default() == "XMAS" {
                    println!("Forward");
                    counter += 1;
                }
                //Search back
                if c_index >= 3 {
                    if l.get(c_index - 3..=c_index).unwrap_or_default() == "SAMX" {
                        println!("Backwards");
                        counter += 1;
                    }
                }
                //search down
                if lines
                    .get(index)
                    .unwrap_or(&"")
                    .get(c_index..=c_index)
                    .unwrap_or_default()
                    .to_string()
                    + lines
                        .get(index + 1)
                        .unwrap_or(&"")
                        .get(c_index..=c_index)
                        .unwrap_or_default()
                    + lines
                        .get(index + 2)
                        .unwrap_or(&"")
                        .get(c_index..=c_index)
                        .unwrap_or_default()
                    + lines
                        .get(index + 3)
                        .unwrap_or(&"")
                        .get(c_index..=c_index)
                        .unwrap_or_default()
                    == "XMAS"
                {
                    counter += 1;
                    println!("Down")
                }
                // down right
                if lines
                    .get(index)
                    .unwrap_or(&"")
                    .get(c_index..=c_index)
                    .unwrap_or_default()
                    .to_string()
                    + lines
                        .get(index + 1)
                        .unwrap_or(&"")
                        .get(c_index + 1..=c_index + 1)
                        .unwrap_or_default()
                    + lines
                        .get(index + 2)
                        .unwrap_or(&"")
                        .get(c_index + 2..=c_index + 2)
                        .unwrap_or_default()
                    + lines
                        .get(index + 3)
                        .unwrap_or(&"")
                        .get(c_index + 3..=c_index + 3)
                        .unwrap_or_default()
                    == "XMAS"
                {
                    counter += 1;
                    println!("Down right")
                }

                if c_index >= 3 {
                    // down left
                    if lines
                        .get(index)
                        .unwrap_or(&"")
                        .get(c_index..=c_index)
                        .unwrap_or_default()
                        .to_string()
                        + lines
                            .get(index + 1)
                            .unwrap_or(&"")
                            .get(c_index - 1..=c_index - 1)
                            .unwrap_or_default()
                        + lines
                            .get(index + 2)
                            .unwrap_or(&"")
                            .get(c_index - 2..=c_index - 2)
                            .unwrap_or_default()
                        + lines
                            .get(index + 3)
                            .unwrap_or(&"")
                            .get(c_index - 3..=c_index - 3)
                            .unwrap_or_default()
                        == "XMAS"
                    {
                        counter += 1;
                        println!("Down left")
                    }
                }

                if index >= 3 {
                    //up
                    if lines
                        .get(index)
                        .unwrap_or(&"")
                        .get(c_index..=c_index)
                        .unwrap_or_default()
                        .to_string()
                        + lines
                            .get(index - 1)
                            .unwrap_or(&"")
                            .get(c_index..=c_index)
                            .unwrap_or_default()
                        + lines
                            .get(index - 2)
                            .unwrap_or(&"")
                            .get(c_index..=c_index)
                            .unwrap_or_default()
                        + lines
                            .get(index - 3)
                            .unwrap_or(&"")
                            .get(c_index..=c_index)
                            .unwrap_or_default()
                        == "XMAS"
                    {
                        counter += 1;
                        println!("up")
                    }

                    //up right
                    if lines
                        .get(index)
                        .unwrap_or(&"")
                        .get(c_index..=c_index)
                        .unwrap_or_default()
                        .to_string()
                        + lines
                            .get(index - 1)
                            .unwrap_or(&"")
                            .get(c_index + 1..=c_index + 1)
                            .unwrap_or_default()
                        + lines
                            .get(index - 2)
                            .unwrap_or(&"")
                            .get(c_index + 2..=c_index + 2)
                            .unwrap_or_default()
                        + lines
                            .get(index - 3)
                            .unwrap_or(&"")
                            .get(c_index + 3..=c_index + 3)
                            .unwrap_or_default()
                        == "XMAS"
                    {
                        counter += 1;
                        println!("up right")
                    }

                    if c_index >= 3 {
                        //up left
                        if lines
                            .get(index)
                            .unwrap_or(&"")
                            .get(c_index..=c_index)
                            .unwrap_or_default()
                            .to_string()
                            + lines
                                .get(index - 1)
                                .unwrap_or(&"")
                                .get(c_index - 1..=c_index - 1)
                                .unwrap_or_default()
                            + lines
                                .get(index - 2)
                                .unwrap_or(&"")
                                .get(c_index - 2..=c_index - 2)
                                .unwrap_or_default()
                            + lines
                                .get(index - 3)
                                .unwrap_or(&"")
                                .get(c_index - 3..=c_index - 3)
                                .unwrap_or_default()
                            == "XMAS"
                        {
                            counter += 1;
                            println!("up left")
                        }
                    }
                }
            }
        }
    }

    println!("{counter}");
}
