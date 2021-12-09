use utils;

fn lines_to_int(lines: &Vec<String>) -> Vec<Vec<u32>>
{
    lines.iter().map(|l| l.chars().map(|m| m.to_digit(10).unwrap()).collect())
        .collect()
}

fn local_minima(data: &Vec<Vec<u32>>, x: usize, y: usize) -> bool
{
    let mut adjacent_locations = Vec::new();

    if (x as i32)-1 >= 0 {
        adjacent_locations.push(data[y][x-1]);
    }
    if x+1 < data[0].len() {
        adjacent_locations.push(data[y][x+1]);
    }
    if (y as i32)-1 >= 0 {
        adjacent_locations.push(data[y-1][x]);
    }
    if y+1 < data.len() {
        adjacent_locations.push(data[y+1][x]);
    }

    for location in adjacent_locations {
        if data[y][x] >= location {
            return false
        }
    }

    true
}

fn part1(data: &Vec<Vec<u32>>) -> u32
{
    let mut minima = Vec::new();

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if local_minima(&data, x, y) {
                minima.push(data[y][x] + 1);
            }
        }
    }

    minima.iter().sum()
}

fn main()
{
    let lines = utils::read_input_file();
    let data = lines_to_int(&lines);

    println!("Part 1: {:?}", part1(&data));
    //println!("Part 2: {:?}", part2(&data));
}
