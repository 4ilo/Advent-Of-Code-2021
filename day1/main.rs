fn lines_to_int(lines: &[String]) -> Vec<u32>
{
    lines.iter().map(|l| l.parse::<u32>().unwrap())
        .collect()
}

fn part1(lines: &[u32]) -> u32
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

fn part2(lines: &[u32]) -> u32
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
    let lines = utils::read_input_file();
    let data = lines_to_int(&lines);

    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data));
}
