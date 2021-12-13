#[macro_use] extern crate scan_fmt;
use std::collections::HashMap;

mod line;

fn data_to_lines(data: &[String]) -> Vec<line::Line>
{
    let mut lines = Vec::new();

    for line in data {
        let (p1, p2) = scan_fmt_some!(line, "{} -> {}", line::Point, line::Point);
        lines.push(line::Line (p1.unwrap(), p2.unwrap()))
    }

    lines
}

fn calc(lines: &[line::Line], skip: bool) -> u32
{
    let mut counter = 0;
    let mut covered_points: HashMap<line::Point, u32> = HashMap::new();

    for line in lines {
        if ! (line.horizontal() || line.vertical()) && skip {
            continue;
        }

        for point in line.points() {
            *(covered_points.entry(point).or_insert(0)) += 1;
        }
    }

    for (_point, number) in covered_points {
        if number >=2 {
            counter += 1;
        }
    }

    counter
}

fn main()
{
    let data = utils::read_input_file();
    let lines = data_to_lines(&data);

    println!("Part 1: {:?}", calc(&lines, true));
    println!("Part 2: {:?}", calc(&lines, false));
}
