fn main() {
    let input = include_str!("../../input.txt");

    let lines = input.lines().collect::<Vec<_>>();

    let mut counter = 0;

    //Skip the first line
    for (index, l) in lines.iter().enumerate().skip(1) {
        for (c_index, c) in l.chars().enumerate().skip(1) {
            if c != 'A' {
                continue;
            }
            let left = lines
                .get(index - 1)
                .unwrap_or(&"")
                .get(c_index - 1..=c_index - 1)
                .unwrap_or_default()
                .to_string()
                + &c.to_string()
                + lines
                    .get(index + 1)
                    .unwrap_or(&"")
                    .get(c_index + 1..=c_index + 1)
                    .unwrap_or_default();

            let right = lines
                .get(index - 1)
                .unwrap_or(&"")
                .get(c_index + 1..=c_index + 1)
                .unwrap_or_default()
                .to_string()
                + &c.to_string()
                + lines
                    .get(index + 1)
                    .unwrap_or(&"")
                    .get(c_index - 1..=c_index - 1)
                    .unwrap_or_default();

            if (left == "MAS" || left == "SAM") && (right == "MAS" || right == "SAM") {
                counter += 1;
            }
        }
    }

    println!("{counter}");
}
