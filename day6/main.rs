use utils;

fn data_to_int(lines: &String) -> Vec<u8>
{
    lines.split(",").map(|l| l.parse::<u8>().unwrap())
        .collect()
}

fn calc(starting_fish: &Vec<u8>, days: u32) -> u64
{
    let mut fish_map = vec![0;9];

    for fish in starting_fish {
        fish_map[*fish as usize] += 1;
    }

    for _day in 0..days {
        let spawn = fish_map[0];
        fish_map.remove(0);
        fish_map.push(spawn);
        fish_map[6] += spawn;
    }

    fish_map.iter().sum()
}

fn main()
{
    let data = utils::read_input_file();
    let lanternfish = data_to_int(&data[0]);

    println!("Part 1: {:?}", calc(&lanternfish, 80));
    println!("Part 2: {:?}", calc(&lanternfish, 256));
}
