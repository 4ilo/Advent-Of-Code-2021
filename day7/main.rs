use utils;

type FuelFunction = fn(&Vec<u32>, u32) -> u32;

fn data_to_int(lines: &String) -> Vec<u32>
{
    lines.split(",").map(|l| l.parse::<u32>().unwrap())
        .collect()
}

fn part1(positions: &Vec<u32>, target: u32) -> u32
{
    let mut sum = 0;

    for position in positions {
        sum += std::cmp::max(position, &target) - std::cmp::min(position, &target);
    }

    sum
}

fn part2(positions: &Vec<u32>, target: u32) -> u32
{
    let mut score = 0;

    for position in positions {
        let diff = std::cmp::max(position, &target) - std::cmp::min(position, &target);

        for i in 0..diff+1 {
            score += i;
        }
    }

    score
}

fn simulate(positions: &Vec<u32>, fuel_function: FuelFunction) -> u32
{
    let mut best_score: u32 = std::u32::MAX;

    for i in 0..*positions.last().unwrap() {
        let score = fuel_function(positions, i);

        if score < best_score {
            best_score = score;
        }
    }

    best_score
}

fn main()
{
    let data = utils::read_input_file();
    let mut positions = data_to_int(&data[0]);
    positions.sort();

    println!("Part 1: {:?}", simulate(&positions, part1));
    println!("Part 2: {:?}", simulate(&positions, part2));
}
