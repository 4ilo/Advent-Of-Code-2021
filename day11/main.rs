use utils;

#[derive(Debug, Clone)]
struct Octopus {
    energy_level: u8,
    flashed: bool,
}

impl Octopus {
    fn flashable(&self) -> bool {
        self.energy_level >= 10 && self.flashed != true
    }

    fn reset(&mut self) {
        self.energy_level = 0;
        self.flashed = false;
    }
}

fn parse_lines(lines: Vec<String>) -> Vec<Vec<Octopus>>
{
    let mut data = Vec::new();

    for line in lines {
        let mut line_data = Vec::new();
        for c in line.chars() {
            line_data.push(Octopus {
                energy_level: c.to_digit(10).unwrap() as u8,
                flashed: false,
            })
        }

        data.push(line_data);
    }

    data
}

fn get_neighbors(field: &mut Vec<Vec<Octopus>>, x: usize, y: usize) -> Vec<(usize, usize)>
{
    let mut neighbors = Vec::new();

    if (x as i32)-1 >= 0 {
        neighbors.push((y, x-1));

        if (y as i32)-1 >= 0 {
            neighbors.push((y-1, x-1));
        }
        if y+1 < field.len() {
            neighbors.push((y+1, x-1));
        }
    }
    if x+1 < field[0].len() {
        neighbors.push((y, x+1));

        if (y as i32)-1 >= 0 {
            neighbors.push((y-1, x+1));
        }
        if y+1 < field.len() {
            neighbors.push((y+1, x+1));
        }
    }
    if (y as i32)-1 >= 0 {
        neighbors.push((y-1, x));
    }
    if y+1 < field.len() {
        neighbors.push((y+1, x));
    }

    neighbors
}

fn flash_if_needed(field: &mut Vec<Vec<Octopus>>, x: usize, y: usize)
{
    if ! field[y][x].flashable() {
        return;
    }

    field[y][x].flashed = true;

    let neighbors = get_neighbors(field, x, y);
    for neighbor in &neighbors {
        field[neighbor.0][neighbor.1].energy_level += 1;
    }

    for neighbor in &neighbors {
        flash_if_needed(field, neighbor.1, neighbor.0);
    }
}

fn part1(mut field: Vec<Vec<Octopus>>, iterations: u32) -> u32
{
    let mut total_flashes = 0;

    for _i in 0..iterations {
        // Increase levels
        for y in 0..field.len() {
            for x in 0..field[0].len() {
                field[y][x].energy_level += 1;
            }
        }

        // Flash required octopuses
        for y in 0..field.len() {
            for x in 0..field[0].len() {
                flash_if_needed(&mut field, x, y);
            }
        }

        // Reset flashed octopuses
        for y in 0..field.len() {
            for x in 0..field[0].len() {
                if field[y][x].flashed {
                    total_flashes += 1;
                    field[y][x].reset();
                }
            }
        }
    }

    total_flashes
}


fn part2(mut field: Vec<Vec<Octopus>>) -> usize
{
    let num_octopuses = field.len() * field[0].len();

    for _i in 0..1000 {
        let mut num_flashes = 0;

        // Increase levels
        for y in 0..field.len() {
            for x in 0..field[0].len() {
                field[y][x].energy_level += 1;
            }
        }

        // Flash required octopuses
        for y in 0..field.len() {
            for x in 0..field[0].len() {
                flash_if_needed(&mut field, x, y);
            }
        }

        // Reset flashed octopuses
        for y in 0..field.len() {
            for x in 0..field[0].len() {
                if field[y][x].flashed {
                    num_flashes += 1;
                    field[y][x].reset();
                }
            }
        }

        if num_flashes == num_octopuses {
            return _i+1;
        }
    }

    0
}

fn main()
{
    let lines = utils::read_input_file();
    let data = parse_lines(lines);

    println!("Part 1: {:?}", part1(data.clone(), 100));
    println!("Part 2: {:?}", part2(data.clone()));
}
