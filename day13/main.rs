#[macro_use] extern crate scan_fmt;

use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn data_to_lines(data: &[String]) -> (HashSet<Point>, Vec<(u32, u32)>)
{
    let mut points = HashSet::new();
    let mut folds = Vec::new();

    for line in data {
        if line.contains("fold") {
            let (axis, index) = scan_fmt_some!(line, "fold along {}={}", char, u32);
            folds.push(match axis.unwrap() {
                'x' => (index.unwrap(), 0),
                'y' => (0, index.unwrap()),
                _ => panic!("Invalid axis"),
            });
        }
        else {
            let coords: Vec<&str> = line.split(',').collect();
            if coords.len() == 2 {
                points.insert(Point{
                    x: coords[0].parse::<i32>().unwrap(),
                    y: coords[1].parse::<i32>().unwrap(),
                });
            }
        }
    }

    (points, folds)
}

fn do_fold(points: &HashSet<Point>, fold: &(u32, u32)) -> HashSet<Point>
{
    let mut paper = HashSet::new();

    for point in points.iter() {
        if fold.1 != 0 && point.y > fold.1 as i32 {
            paper.insert(Point {
                x: point.x,
                y: point.y - (point.y - fold.1 as i32)*2,
            });
        }
        else if fold.0 != 0 && point.x > fold.0 as i32 {
            paper.insert(Point {
                x: point.x - (point.x - fold.0 as i32)*2,
                y: point.y,
            });
        }
        else {
            paper.insert(Point {
                x: point.x,
                y: point.y,
            });
        }
    }

    paper
}

fn part2(points: &HashSet<Point>, folds: &[(u32, u32)])
{
    let mut paper = points.clone();

    for fold in folds {
        paper = do_fold(&paper, fold);
    }

    // Print out paper
    println!("Part2:");
    for y in 0..6 {
        for x in 0..39 {
            if paper.contains(&Point {x, y}) {
                print!("#");
            }
            else {
                print!(".");
            }
        }

        println!()
    }
}

fn main()
{
    let data = utils::read_input_file();
    let (points, folds) = data_to_lines(&data);

    println!("Part 1: {:?}", do_fold(&points, &folds[0]).len());
    part2(&points, &folds);
}
