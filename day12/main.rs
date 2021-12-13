use std::collections::HashMap;
use std::collections::HashSet;

fn parse_lines(lines: Vec<String>) -> HashMap<String, Vec<String>>
{
    let mut graph = HashMap::new();

    for line in lines {
        let splitted_line: Vec<String> = line.split('-').map(|l| l.into()).collect();
        graph.entry(splitted_line[0].clone()).or_insert_with(Vec::new).push(splitted_line[1].clone());
        graph.entry(splitted_line[1].clone()).or_insert_with(Vec::new).push(splitted_line[0].clone());
    }

    graph
}

fn find_path(graph: &HashMap<String, Vec<String>>, node: &str, visited: &HashSet<String>, double: bool) -> u32
{
    if node == "end" {
        return 1;
    }

    let mut new_visited = visited.clone();
    if node.to_lowercase() == *node {
        new_visited.insert(node.to_string());
    }

    let mut count = 0;
    for dest in graph.get(node).unwrap() {
        if dest == "start" {
            // Never visit 'start' twice
            continue
        };

        if visited.contains(dest) {
            if double {
                // We already visited dest
                continue;
            }
            count += find_path(graph, dest, &new_visited, true);
        }
        else {
            count += find_path(graph, dest, &new_visited, double);
        }
    }

    count
}

fn part1(graph: &HashMap<String, Vec<String>>) -> u32
{
    let visited: HashSet<String> = HashSet::new();
    find_path(graph, "start", &visited, true)
}

fn part2(graph: &HashMap<String, Vec<String>>) -> u32
{
    let visited: HashSet<String> = HashSet::new();
    find_path(graph, "start", &visited, false)
}

fn main()
{
    let lines = utils::read_input_file();
    let graph = parse_lines(lines);

    println!("Part 1: {:?}", part1(&graph));
    println!("Part 2: {:?}", part2(&graph));
}
