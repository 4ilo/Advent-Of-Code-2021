#[macro_use] extern crate scan_fmt;

fn parse_data(data: &str) -> ((i32, i32), (i32, i32))
{
    let (x1, x2, y1, y2) = scan_fmt_some!(data, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32);

    ((x1.unwrap(), y1.unwrap()), (x2.unwrap(), y2.unwrap()))
}

fn part1(p1: (i32, i32)) -> i32
{
    (0..p1.1.abs()).sum()
}

fn step(probe: &mut (i32, i32), velocity: &mut (i32, i32))
{
    // Move
    probe.0 += velocity.0;
    probe.1 += velocity.1;

    // Update velocity
    match velocity.0.cmp(&0) {
        std::cmp::Ordering::Less => velocity.0 += 1,
        std::cmp::Ordering::Greater => velocity.0 -= 1,
        std::cmp::Ordering::Equal => (),
    }

    velocity.1 -= 1;
}

fn in_area(probe: (i32, i32), p1: (i32, i32), p2: (i32, i32)) -> bool
{
    probe.0 >= p1.0 && probe.0 <= p2.0 &&
    probe.1 >= p1.1 && probe.1 <= p2.1
}

fn out_of_range(probe: (i32, i32),  p1: (i32, i32), p2: (i32, i32)) -> bool
{
    probe.0 > p2.0 || probe.1 < p1.1
}

fn simulate(p1: (i32, i32), p2: (i32, i32), initial_velocity: (i32, i32)) -> bool
{
    let mut max_y = 0;
    let mut probe: (i32, i32) = (0, 0);
    let mut velocity = initial_velocity;

    while ! in_area(probe, p1, p2) {
        if out_of_range(probe, p1, p2) {
            return false;
        }

        max_y = std::cmp::max(max_y, probe.1);
        step(&mut probe, &mut velocity);
    }

    true
}

fn part2(p1: (i32, i32), p2: (i32, i32)) -> u32
{
    let iter = 400;
    let mut target_hit = 0;

    for x in 0..iter {
        for y in -iter..iter {
            if simulate(p1, p2, (x, y)) {
                target_hit += 1;
            }
        }
    }

    target_hit
}


fn main()
{
    let lines = utils::read_input_file();
    let (p1, p2) = parse_data(&lines[0]);

    println!("Part 1: {:?}", part1(p1));
    println!("Part 2: {:?}", part2(p1, p2));
}
