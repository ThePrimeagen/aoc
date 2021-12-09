#[derive(Debug, Copy, Clone)]
pub enum BoardSlot {
    Unmarked(i32),
    Marked(i32),
}

impl BoardSlot {
    fn is_equal(&self, value: i32) -> bool {
        return match self {
            BoardSlot::Unmarked(v) => {
                if *v == value {
                    true
                } else {
                    false
                }
            },
            BoardSlot::Marked(v) => {
                if *v == value {
                    true
                } else {
                    false
                }
            }
        };
    }
}

#[derive(Debug)]
struct Board {
    board: [[BoardSlot; 5]; 5],
}

impl Board {
    pub fn mark(&mut self, value: i32) -> bool {
        for col in 0..5 {
            for row in 0..5 {
                if self.board[col][row].is_equal(value) {
                    self.board[col][row] = BoardSlot::Marked(value);
                    return self.check_for_win(col, row);
                }
            }
        }
        return false;
    }

    pub fn check_for_win(&self, col: usize, row: usize) -> bool {
        let mut all_true = true;
        for r in 0..5 {
            match self.board[col][r] {
                BoardSlot::Unmarked(_) => {
                    all_true = false;
                    break;
                },
                _ => {}
            }
            match self.board[r][row] {
                BoardSlot::Unmarked(_) => {
                    all_true = false;
                    break;
                },
                _ => {}
            }
        }

        return all_true;
    }

    pub fn values(&self) -> impl Iterator<Item = BoardSlot> + '_ {
        let mut idx = 0;
        return std::iter::from_fn(move || {
            if idx >= 25 {
                return None;
            }
            let slot = self.board[idx % 5][idx / 5];
            idx += 1;

            return Some(slot);
        });
    }
}

fn parse_input(line: &str) -> Result<Vec<i32>, std::io::Error> {
    return Ok(
        line.split(",")
        .map(|item| item.parse::<i32>().unwrap())
        .collect());
}

fn parse_board(vals: &[&str]) -> Result<Option<Board>, std::io::Error> {
    if vals.len() < 5 {
        return Ok(None);
    }

    let out = Board {
        board: [[BoardSlot::Unmarked(0); 5]; 5],
    };

    let (out, _) = vals.iter()
        .map(|line| line.split_whitespace().filter(|x| !x.is_empty()))
        .flatten()
        .map(|item| item.parse::<i32>().unwrap())
        .fold((out, 0), |(mut out, idx), value| {
            out.board[idx % 5][idx / 5] = BoardSlot::Unmarked(value);
            return (out, idx + 1);
        });

    return Ok(Some(out));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = include_str!("day4_1.input").lines().
        filter(|x| !x.is_empty()).collect();

    let input = parse_input(lines[0])?;
    let mut boards: Vec<Board> = lines[1..].chunks(5).map(|chunk| {
        return parse_board(chunk).unwrap().unwrap();
    }).collect();


    let mut total_cheat_because_i_suck_at_rust: Option<i32> = None;
    for i in input {
        if boards.len() == 1 {
            if boards[0].mark(i) {
                println!("WINNER BOARD({}) {:?}", i, boards[0]);
                total_cheat_because_i_suck_at_rust = Some(i * boards[0].values().map(|x| match x {
                    BoardSlot::Unmarked(v) => v,
                    _ => 0,
                }).sum::<i32>());
                break;
            }
        } else {
            boards = boards.into_iter().fold(vec![], |mut boards, mut board| {

                if !board.mark(i) {
                    boards.push(board);
                }

                return boards;
            });
        }
        println!("Board Count {} from input {}", boards.len(), i);

    }

    match total_cheat_because_i_suck_at_rust {
        Some(v) => {
            println!("GOT IT {}, Brought to you by Rustlang:tm:", v)
        },
        _ => println!("You suck")
    }

    return Ok(());
}


