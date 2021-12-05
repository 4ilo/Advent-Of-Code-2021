#[macro_use] extern crate scan_fmt;

use utils;

#[derive(Debug)]
struct Command {
    direction: String,
    unit: i32,
}

fn parse_lines(lines: Vec<String>) -> Vec<Command>
{
    let mut data = Vec::new();

    for line in lines {
        let (direction, unit) = scan_fmt_some!(&line, "{} {d}", String, i32);
        data.push(Command {
            direction: direction.unwrap(),
            unit: unit.unwrap(),
        })
    }

    data
}

fn get_direction(direction_str: &String) -> (i32, i32)
{
    match direction_str.as_str() {
        "forward" => return (1, 0),
        "down" => return (0, 1),
        "up" => return (0, -1),
        _ => return (0, 0),
    }
}

fn part1(commands: &Vec<Command>) -> i32
{
    let (mut x, mut y) = (0, 0);

    for command in commands {
        let dir = get_direction(&command.direction);
        x += dir.0 * command.unit;
        y += dir.1 * command.unit;
    }

    x*y
}

fn part2(commands: &Vec<Command>) -> i32
{
    let (mut x, mut y) = (0, 0);
    let mut aim = 0;

    for command in commands {
        match command.direction.as_str() {
            "down" => aim += command.unit,
            "up" => aim -= command.unit,
            "forward" => { x += command.unit; y += aim * command.unit },
            _ => println!("Invalid command"),
        }
    }

    x*y
}

fn main()
{
    let lines = utils::read_input_file();
    let data = parse_lines(lines);

    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data));
}
