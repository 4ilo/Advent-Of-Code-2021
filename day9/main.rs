use utils;

fn lines_to_int(lines: &Vec<String>) -> Vec<Vec<u32>>
{
    lines.iter().map(|l| l.chars().map(|m| m.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_neighbors(data: &Vec<Vec<u32>>, x: usize, y: usize) -> Vec<(usize, usize)>
{
    let mut neighbors = Vec::new();

    if (x as i32)-1 >= 0 {
        neighbors.push((y, x-1));
    }
    if x+1 < data[0].len() {
        neighbors.push((y, x+1));
    }
    if (y as i32)-1 >= 0 {
        neighbors.push((y-1, x));
    }
    if y+1 < data.len() {
        neighbors.push((y+1, x));
    }

    neighbors
}

fn local_minima(data: &Vec<Vec<u32>>, x: usize, y: usize) -> bool
{
    for neighbor in get_neighbors(&data, x, y) {
        if data[y][x] >= data[neighbor.0][neighbor.1] {
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

fn get_basin_size(data: &Vec<Vec<u32>>, seen: &mut Vec<Vec<bool>>, y: usize, x: usize) -> u32
{
    let mut size = 1;

    if data[y][x] >= 9 {
        return 0;
    }

    seen[y][x] = true;

    for neighbor in get_neighbors(data, x, y) {
        if seen[neighbor.0][neighbor.1] {
            continue;
        }

        size += get_basin_size(data, seen, neighbor.0, neighbor.1);
    }

    size
}

fn part2(data: &Vec<Vec<u32>>) -> u32
{
    let mut minima = Vec::new();

    // Get the minima
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if local_minima(&data, x, y) {
                minima.push((y, x));
            }
        }
    }

    let mut basins = Vec::new();
    let mut seen = vec![vec![false;data[0].len()];data.len()];

    for minimum in minima {
        basins.push(get_basin_size(&data, &mut seen, minimum.0, minimum.1));
    }

    basins.sort();
    basins.iter().rev().take(3).product()
}

fn main()
{
    let lines = utils::read_input_file();
    let data = lines_to_int(&lines);

    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data));
}
