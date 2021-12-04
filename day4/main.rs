use std::io::BufRead;

#[derive(Debug,Clone)]
struct Board {
    finished: bool,
    lines: Vec<Vec<u32>>,
    marked: Vec<Vec<u8>>,
    marked_flipped: Vec<Vec<u8>>,
}

impl Board {
    fn new() -> Board {
        Board {
            finished: false,
            lines: Vec::new(),
            marked: vec![vec![0; 5]; 5],
            marked_flipped: vec![vec![0; 5]; 5],
        }
    }

    fn mark(&mut self, number: u32) {
        for (i, line) in self.lines.iter().enumerate() {
            for (j, num) in line.iter().enumerate() {
                if number == *num {
                    self.marked[i][j] = 1;
                    self.marked_flipped[j][i] = 1;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        for line in &self.marked {
            if line.iter().sum::<u8>() == 5 {
                return true;
            }
        }

        for line in &self.marked_flipped {
            if line.iter().sum::<u8>() == 5 {
                return true;
            }
        }

        false
    }

    fn calc_score(&self, last_draw: u32) -> u32 {
        let mut score = 0;

        for (i, line) in self.lines.iter().enumerate() {
            for (j, num) in line.iter().enumerate() {
                if self.marked[i][j] != 1 {
                    score += num;
                }
            }
        }

        score * last_draw
    }
}

fn get_input_filename() -> String
{
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Missing input file.");
        std::process::exit(1);
    }

    args[1].clone()
}

fn data_from_file(filename: String) -> (Vec<u32>, Vec<Board>)
{
    let file = std::fs::File::open(filename).expect("no such file");
    let buf = std::io::BufReader::new(file);

    let mut numbers = Vec::new();
    let mut boards = Vec::new();
    let mut current_board = Board::new();

    for line in buf.lines() {
        let line_data = line.unwrap();
        if numbers.len() == 0 {
            // First input row
            numbers = line_data
                .split(",")
                .map(|l| l.parse::<u32>().unwrap())
                .collect();

            continue;
        }
        if line_data.len() == 0 {
            // Whiteline
            if current_board.lines.len() != 0 {
                boards.push(current_board);
                current_board = Board::new();
            }
            continue;
        }

        current_board.lines.push(
            line_data.split_whitespace()
                .map(|l| l.parse::<u32>().unwrap())
                .collect()
        )
    }

    boards.push(current_board);

    (numbers, boards)
}


fn part1(numbers: &Vec<u32>, boards: &mut Vec<Board>) -> u32
{
    for draw in numbers {
        for board in &mut *boards {
            board.mark(*draw);

            if board.is_winner() {
                let score = board.calc_score(*draw);
                return score;
            }
        }
    }

    panic!("Unable to find solution");
}

fn part2(numbers: &Vec<u32>, boards: &mut Vec<Board>) -> u32
{
    let mut finished = 0;
    let len = boards.len();

    for draw in numbers {
        for board in &mut *boards {
            if board.finished {
                continue;
            }

            board.mark(*draw);

            if board.is_winner() {
                board.finished = true;
                finished += 1;
            }

            if finished == len {
                return board.calc_score(*draw);
            }
        }
    }

    panic!("Unable to find solution");
}


fn main()
{
    let filename = get_input_filename();
    let (numbers, boards) = data_from_file(filename);

    println!("Part 1: {:?}", part1(&numbers, &mut boards.clone()));
    println!("Part 2: {:?}", part2(&numbers, &mut boards.clone()));
}
