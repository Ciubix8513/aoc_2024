fn check_continuos(v: &[Option<usize>]) -> bool {
    let mut first_none = false;

    for i in v {
        if i.is_none() {
            first_none = true;
        }

        if i.is_some() && first_none {
            return false;
        }
    }

    true
}

fn main() {
    let input = include_str!("../../input.txt").trim_end();

    let mut file_id = 0;

    let mut fs = Vec::new();

    for (ind, c) in input.char_indices() {
        if ind % 2 == 0 {
            //File
            for _ in 0..(c.to_string().parse().unwrap()) {
                fs.push(Some(file_id));
            }
            file_id += 1;
        } else {
            //free space
            for _ in 0..(c.to_string().parse().unwrap()) {
                fs.push(None);
            }
        }
    }

    let binding = fs
        .iter()
        .enumerate()
        .filter(|(_, i)| i.is_none())
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let mut empties = binding.iter();

    for i in (0..fs.len()).rev() {
        //check if continuous
        if check_continuos(&fs) {
            break;
        }

        if fs[i].is_some() {
            if let Some(e) = empties.next() {
                fs.swap(i, *e);
            } else {
                break;
            }
        }
    }

    let checksum = fs
        .iter()
        .enumerate()
        .fold(0, |a, (ind, i)| a + ind * i.unwrap_or_default());

    println!("{checksum}");
}
