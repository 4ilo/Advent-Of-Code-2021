use utils;

fn lines_to_int(lines: &Vec<String>) -> Vec<Vec<usize>>
{
    let mut data = Vec::new();

    for line in lines {
        let mut data_line = Vec::new();

        for c in line.chars() {
            let token = match c {
                '(' => 0,
                '[' => 1,
                '{' => 2,
                '<' => 3,
                ')' => 4,
                ']' => 5,
                '}' => 6,
                '>' => 7,
                _ => panic!("Unknown token"),
            };

            data_line.push(token);
        }

        data.push(data_line);
    }

    data
}

fn get_score(token: usize) -> u32
{
    match token {
        4 => 3,
        5 => 57,
        6 => 1197,
        7 => 25137,
        _ => panic!("Invalid token for scoring"),
    }
}

fn calculate(data: &Vec<Vec<usize>>) -> (u32, usize)
{
    let mut part1 = 0;
    let mut scores = Vec::new();

    for line in data {
        let mut corrupt = false;
        let mut opened = Vec::new();

        for token in line {
            if *token < 4 {
                opened.push(*token);
            }
            else {
                if *token-4 == opened.pop().unwrap() {
                    continue;
                }
                else {
                    corrupt = true;
                    part1 += get_score(*token);
                    break
                }
            }
        }

        if ! corrupt {
            let mut score = 0;

            for token in opened.iter().rev() {
                score *= 5;
                score += token+1;
            }

            scores.push(score);
        }
    }

    scores.sort();
    (part1, scores[scores.len()/2])
}

fn main()
{
    let lines = utils::read_input_file();
    let data = lines_to_int(&lines);

    let (part1, part2) = calculate(&data);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
