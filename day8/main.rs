use utils;

#[derive(Debug,Clone)]
struct Entry {
    signal_patterns: Vec<String>,
    output_values: Vec<String>,
}

fn parse_lines(lines: Vec<String>) -> Vec<Entry>
{
    let mut data = Vec::new();

    for line in lines {
        let mut foo = line.split(" | ");
        data.push(Entry {
            signal_patterns: foo.nth(0).unwrap().split(" ").map(|l| l.to_string()).collect(),
            output_values: foo.nth(0).unwrap().split(" ").map(|l| l.to_string()).collect(),
        })
    }

    data
}

fn part1(data: &Vec<Entry>) -> u32
{
    let (mut one, mut four, mut seven, mut eight) = (0, 0, 0, 0);

    for entry in data {
        for value in &entry.output_values {
            let len = value.len();

            if len == 2 {
                one += 1;
            }
            else if len == 4 {
                four += 1;
            }
            else if len == 3 {
                seven += 1;
            }
            else if len == 7 {
                eight += 1;
            }
        }
    }

    one + four + seven + eight
}

fn sort_string(input: &String) -> String
{
    let mut tmp = input.chars().collect::<Vec<char>>();
    tmp.sort();
    tmp.iter().collect()
}

fn digit_in_pattern(digit: &String, pattern: &String) -> bool
{
    for c in digit.chars() {
        if !pattern.contains(c) {
            return false
        }
    }

    return true
}

fn get_number(map: &Vec<String>, output_values: &Vec<String>) -> u32
{
    let mut number = 0;
    for (i, value) in output_values.iter().enumerate() {
        let value_sorted = sort_string(value);

        for (j, digit) in map.iter().enumerate() {
            if value_sorted == *digit {
                number += usize::pow(10, (3-i) as u32)*j
            }
        }
    }

    number as u32
}

fn part2(data: &Vec<Entry>) -> u32
{
    let mut sum = 0;
    for entry in data {
        let mut two_and_five = Vec::new();
        let mut digit_map: Vec<String> = vec![String::new();10];

        let mut tmp_entry = entry.clone();
        tmp_entry.signal_patterns.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
        for pattern in tmp_entry.signal_patterns {
            let len = pattern.len();
            let sorted_pattern = sort_string(&pattern);

            if len == 2 {
                digit_map[1] = sorted_pattern;
            }
            else if len == 4 {
                digit_map[4] = sorted_pattern;
            }
            else if len == 3 {
                digit_map[7] = sorted_pattern;
            }
            else if len == 7 {
                digit_map[8] = sorted_pattern;
            }
            else if len == 5 {
                // 2, 3 or 5
                if digit_in_pattern(&digit_map[1], &pattern) {
                    // 3
                    digit_map[3] = sorted_pattern;
                }
                else {
                    two_and_five.push(sorted_pattern);
                }
            }
            else if len == 6 {
                // 0, 6 or 9
                if digit_in_pattern(&digit_map[4], &pattern) {
                    // 9
                    digit_map[9] = sorted_pattern;
                }
                else if digit_in_pattern(&digit_map[7], &pattern) {
                    // 0
                    digit_map[0] = sorted_pattern;
                }
                else {
                    // 6
                    digit_map[6] = sorted_pattern;
                }
            }
        }

        // Fixup difference between 2 and 5
        if digit_in_pattern(&two_and_five[0], &digit_map[6]) {
            digit_map[5] = two_and_five[0].clone();
            digit_map[2] = two_and_five[1].clone();
        }
        else {
            digit_map[5] = two_and_five[1].clone();
            digit_map[2] = two_and_five[0].clone();
        }

        // Decode output values
        sum += get_number(&digit_map, &entry.output_values);
    }

    sum
}

fn main()
{
    let lines = utils::read_input_file();
    let data = parse_lines(lines);

    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data));
}
