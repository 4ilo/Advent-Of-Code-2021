use std::io::BufRead;

fn get_input_filename() -> String
{
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Missing input file.");
        std::process::exit(1);
    }

    args[1].clone()
}

fn lines_from_file(filename: String) -> Vec<u32>
{
    let file = std::fs::File::open(filename).expect("no such file");
    let buf = std::io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<u32>().unwrap())
        .collect()
}

fn part1(lines: &Vec<u32>) -> u32
{
    let mut last_value = 0;
    let mut increase_count = 0;

    for line in lines {
        if last_value == 0 {
            last_value = *line;
            continue;
        }

        if *line > last_value {
            increase_count += 1;
        }

        last_value = *line;
    }

    increase_count
}

fn part2(lines: &Vec<u32>) -> u32
{
    let mut last_value = 0;
    let mut increase_count = 0;

    for i in 0..lines.len() {
        if i+2 >= lines.len() {
            break;
        }

        let sum = lines[i] + lines[i+1] + lines[i+2];

        if last_value == 0 {
            last_value = sum;
            continue;
        }

        if sum > last_value {
            increase_count += 1;
        }

        last_value = sum;
    }

    increase_count
}

fn main()
{
    let filename = get_input_filename();
    let lines = lines_from_file(filename);

    println!("Part 1: {:?}", part1(&lines));
    println!("Part 2: {:?}", part2(&lines));
}
