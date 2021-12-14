#[macro_use] extern crate scan_fmt;
use std::collections::HashMap;
use substring::Substring;

fn parse_lines(data: &[String]) -> (String, HashMap<String, String>)
{
    let mut template = String::new();
    let mut insertion_rules = HashMap::new();

    for line in data {
        if line.contains("->") {
            let (left, right) = scan_fmt_some!(line, "{} -> {}", String, String);
            insertion_rules.insert(left.unwrap(), right.unwrap());
        }
        else if !line.is_empty() {
            template = line.clone();
        }
    }

    (template, insertion_rules)
}

fn grow_polymer(template: &str, insertion_rules: &HashMap<String, String>) -> String
{
    let mut new_template = String::new();

    for i in 0..template.len() {
        new_template += template.substring(i, i+1);
        if i < template.len()-1 {
            new_template += insertion_rules.get(template.substring(i, i+2)).unwrap();
        }
    }

    new_template
}

fn calc(template: &str, insertion_rules: &HashMap<String, String>, steps: u32) -> u32
{
    let mut polymer = template.to_string();

    for _i in 0..steps {
        polymer = grow_polymer(&polymer, insertion_rules);
    }

    let mut char_map: HashMap<char, u32> = HashMap::new();
    for c in polymer.chars() {
        let value = char_map.entry(c).or_insert(0_u32);
        *value += 1;
    }

    let mut sorted_values: Vec<u32> = char_map.into_values().collect();
    sorted_values.sort_unstable();
    sorted_values.last().unwrap() - sorted_values.first().unwrap()
}

fn main()
{
    let lines = utils::read_input_file();
    let (template, insertion_rules) = parse_lines(&lines);

    println!("Part 1: {:?}", calc(&template, &insertion_rules, 10));
    //println!("Part 1: {:?}", calc(&template, &insertion_rules, 40));
}
