fn main() {
    let input = include_str!("../../input.txt").trim_end();

    let mut file_id = 0;

    let mut fs = Vec::new();

    for (ind, c) in input.char_indices() {
        if ind % 2 == 0 {
            //File
            fs.push(
                (0..(c.to_string().parse().unwrap()))
                    .map(|_| Some(file_id))
                    .collect::<Vec<_>>(),
            );
            file_id += 1;
        } else {
            //free space
            fs.push(
                (0..(c.to_string().parse().unwrap()))
                    .map(|_| None)
                    .collect::<Vec<_>>(),
            );
        }
    }

    for i in (0..fs.len()).rev() {
        if fs[i].get(0).unwrap_or(&None).is_none() {
            continue;
        }

        let req_len = fs[i].len();

        let mut found_ind = 0;
        let mut additional_len = 0;

        //try to find a spot
        for (ind, s) in fs
            .iter()
            .enumerate()
            .filter(|(_, i)| i.get(0).unwrap_or(&None).is_none())
        {
            if s.len() >= req_len {
                found_ind = ind;
                additional_len = s.len() - req_len;
                break;
            }
        }

        //There's always no free space at the beginning
        if found_ind == 0 || found_ind > i {
            continue;
        }

        let file = fs.remove(i);
        fs.insert(i, (0..req_len).map(|_| None).collect());
        fs.remove(found_ind);

        fs.insert(found_ind, file);
        if additional_len != 0 {
            fs.insert(found_ind + 1, (0..additional_len).map(|_| None).collect());
        }
    }

    let checksum = fs
        .iter()
        .flatten()
        .enumerate()
        .fold(0, |a, (ind, i)| a + ind * i.unwrap_or_default());

    println!("{checksum}");
}
