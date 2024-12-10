fn index_map(y: usize, x: usize, map: &[Vec<u32>]) -> u32 {
    *map.get(y).unwrap_or(&Vec::new()).get(x).unwrap_or(&255)
}

fn search_path(pos: (usize, usize), map: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let current = map[pos.0][pos.1];
    if current == 9 {
        //Return current position if reached the end
        return vec![pos];
    }

    let mut res = Vec::new();

    //Search adjacent map locations

    if index_map(pos.0 + 1, pos.1, map)
        .checked_sub(current)
        .unwrap_or_default()
        == 1
    {
        res.extend(&search_path((pos.0 + 1, pos.1), map));
    }
    if index_map(pos.0, pos.1 + 1, map)
        .checked_sub(current)
        .unwrap_or_default()
        == 1
    {
        res.extend(&search_path((pos.0, pos.1 + 1), map));
    }
    if index_map(pos.0.checked_sub(1).unwrap_or(0xffff), pos.1, map)
        .checked_sub(current)
        .unwrap_or_default()
        == 1
    {
        res.extend(&search_path((pos.0 - 1, pos.1), map));
    }
    if index_map(pos.0, pos.1.checked_sub(1).unwrap_or(0xffff), map)
        .checked_sub(current)
        .unwrap_or_default()
        == 1
    {
        res.extend(&search_path((pos.0, pos.1 - 1), map));
    }

    res
}

fn main() {
    let input = include_str!("../../input.txt");

    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    for (y, m) in map.iter().enumerate() {
        for (x, o) in m.iter().enumerate() {
            if *o != 0 {
                continue;
            }

            let mut ends = search_path((y, x), &map);
            ends.sort_unstable();
            ends.dedup();

            sum += ends.len();
        }
    }

    println!("{sum}");
}
