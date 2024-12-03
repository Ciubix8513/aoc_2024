#[derive(PartialEq, Eq)]
enum State {
    Increase,
    Decrease,
    Fail,
    None,
}

fn main() {
    let o = include_str!("../../input.txt")
        .lines()
        .map(|i| {
            i.split(" ")
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
                .windows(2)
                .map(|i| {
                    let dif = (i[0] - i[1]).abs();
                    if dif >= 1 && dif <= 3 {
                        if i[0] > i[1] {
                            State::Decrease
                        } else {
                            State::Increase
                        }
                    } else {
                        State::Fail
                    }
                })
                .fold((1, State::None), |mut a, i| {
                    match i {
                        State::Increase => {
                            if a.1 == State::Decrease {
                                a.0 = 0;
                            }
                        }
                        State::Decrease => {
                            if a.1 == State::Increase {
                                a.0 = 0;
                            }
                        }
                        State::Fail => {
                            a.0 = 0;
                        }
                        State::None => todo!(),
                    }
                    a.1 = i;
                    a
                })
                .0
        })
        .fold(0, |a, i| a + i);

    println!("{o}")
}
