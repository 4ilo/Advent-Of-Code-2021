#[macro_use] extern crate scan_fmt;
use std::collections::HashMap;
use substring::Substring;

fn parse_lines(data: &[String]) -> (String, HashMap<String, char>)
{
    let mut template = String::new();
    let mut insertion_rules = HashMap::new();

    for line in data {
        if line.contains("->") {
            let (left, right) = scan_fmt_some!(line, "{} -> {}", String, char);
            insertion_rules.insert(left.unwrap(), right.unwrap());
        }
        else if !line.is_empty() {
            template = line.clone();
        }
    }

    (template, insertion_rules)
}

fn calc(template: &str, insertion_rules: &HashMap<String, char>, steps: u32) -> u64
{
    let mut char_map: HashMap<char, u64> = HashMap::new();
    let mut polymer_map: HashMap<String, u64> = HashMap::new();

    // Construct initial map
    for i in 0..template.len()-1 {
        *polymer_map.entry(template.substring(i, i+2).into()).or_insert(0) += 1;
    }

    // Create map to store the available amount for each element
    for c in template.chars() {
        *(char_map.entry(c).or_insert(0_u64)) += 1;
    }

    for _i in 0..steps {
        for (key, value) in polymer_map.clone() {
            let token1 = format!("{}{}", key.substring(0, 1), insertion_rules.get(&key).unwrap());
            let token2 = format!("{}{}", insertion_rules.get(&key).unwrap(), key.substring(1, 2));

            // Count the chars in the string
            *char_map.entry(*insertion_rules.get(&key).unwrap()).or_insert(0_u64) += value;

            *polymer_map.entry(token1).or_insert(0) += value;
            *polymer_map.entry(token2).or_insert(0) += value;
            *polymer_map.entry(key).or_insert(0) -= value;
        }
    }

    let mut sorted_values: Vec<u64> = char_map.into_values().collect();
    sorted_values.sort_unstable();

    sorted_values.last().unwrap() - sorted_values.first().unwrap()
}

fn main()
{
    let lines = utils::read_input_file();
    let (template, insertion_rules) = parse_lines(&lines);

    println!("Part 1: {:?}", calc(&template, &insertion_rules, 10));
    println!("Part 1: {:?}", calc(&template, &insertion_rules, 40));
}
