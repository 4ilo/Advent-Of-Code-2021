use std::collections::HashSet;
use std::collections::HashMap;

fn parse_lines(lines: &[String]) -> Vec<Vec<u32>>
{
    let mut data = Vec::new();

    for line in lines {
        let line_data = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        data.push(line_data);
    }

    data
}

fn get_neighbors(len_x: usize, len_y: usize, x: usize, y: usize) -> Vec<(usize, usize)>
{
    let mut neighbors = Vec::new();

    if (x as i32) > 0 {
        neighbors.push((y, x-1));
    }
    if x+1 < len_x {
        neighbors.push((y, x+1));
    }
    if (y as i32) > 0 {
        neighbors.push((y-1, x));
    }
    if y+1 < len_y {
        neighbors.push((y+1, x));
    }

    neighbors
}

fn get_next_node(unvisited: &HashSet<(usize, usize)>, tentative_distance: &HashMap<(usize, usize), u32>) -> (usize, usize)
{
    let mut closest_node = unvisited.iter().nth(0).unwrap();

    for node in unvisited {
        if tentative_distance.get(node).unwrap_or(&u32::MAX) < tentative_distance.get(closest_node).unwrap_or(&u32::MAX) {
            closest_node = node;
        }
    }

    closest_node.clone()
}

fn part1(lines: &[Vec<u32>]) -> u32
{
    let len_x = lines[0].len();
    let len_y = lines.len();

    let mut unvisited_set = HashSet::new();
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            unvisited_set.insert((x, y));
        }
    }

    let mut tentative_distance: HashMap<(usize, usize), u32> = HashMap::new();
    tentative_distance.insert((0, 0), 0);

    while ! unvisited_set.is_empty() {
        let current_node = get_next_node(&unvisited_set, &tentative_distance);
        let neighbors = get_neighbors(len_x, len_y, current_node.0, current_node.1);
        for (n_y, n_x) in neighbors {
            if !unvisited_set.contains(&(n_x, n_y)) {
                continue;
            }

            let current_score = tentative_distance.get(&(n_x, n_y)).unwrap_or(&u32::MAX);
            let score  = tentative_distance.get(&current_node).unwrap_or(&u32::MAX) + lines[n_y][n_x];
            *(tentative_distance.entry((n_x, n_y)).or_insert(u32::MAX)) = std::cmp::min(score, *current_score);
        }

        unvisited_set.remove(&current_node);

        if current_node == (len_x-1, len_y-1) {
            break;
        }
    }

    *tentative_distance.get(&(len_x-1, len_y-1)).unwrap()
}

fn get_risk_level(risk_levels: &[Vec<u32>], x: usize, y: usize) -> u32
{
    let norm_x = x % risk_levels[0].len();
    let chunk_x = x / risk_levels[0].len();

    let norm_y = y % risk_levels[0].len();
    let chunk_y = y / risk_levels[0].len();

    let level = risk_levels[norm_y][norm_x] + chunk_x as u32 + chunk_y as u32;

    if level > 9 {
        level % 9
    }
    else {
        level
    }
}

fn part2(lines: &[Vec<u32>]) -> u32
{
    let len_x = lines[0].len() * 5;
    let len_y = lines.len() * 5;

    let mut unvisited_set = HashSet::new();
    for y in 0..len_y {
        for x in 0..len_x {
            unvisited_set.insert((x, y));
        }
    }

    let mut tentative_distance: HashMap<(usize, usize), u32> = HashMap::new();
    tentative_distance.insert((0, 0), 0);

    while ! unvisited_set.is_empty() {
        let current_node = get_next_node(&unvisited_set, &tentative_distance);
        let neighbors = get_neighbors(len_x, len_y, current_node.0, current_node.1);
        for (n_y, n_x) in neighbors {
            if !unvisited_set.contains(&(n_x, n_y)) {
                continue;
            }

            let current_score = tentative_distance.get(&(n_x, n_y)).unwrap_or(&u32::MAX);
            let score  = tentative_distance.get(&current_node).unwrap_or(&u32::MAX) + get_risk_level(lines, n_x, n_y);
            *(tentative_distance.entry((n_x, n_y)).or_insert(u32::MAX)) = std::cmp::min(score, *current_score);
        }

        unvisited_set.remove(&current_node);

        if current_node == (len_x-1, len_y-1) {
            break;
        }
    }

    *tentative_distance.get(&(len_x-1, len_y-1)).unwrap()
}

fn main()
{
    let lines = utils::read_input_file();
    let data = parse_lines(&lines);

    println!("Part 1: {:?}", part1(&data));
    //println!("Part 2: {:?}", part2(&data));
}
