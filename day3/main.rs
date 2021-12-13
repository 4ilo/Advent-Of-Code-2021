
fn vec_sum(data: &[String]) -> Vec<usize>
{
    let mut sum = vec![0; data[0].len()];

    for line in data {
        for (i, el) in line.chars().enumerate() {
            if el == '1' {
                sum[i] += 1;
            }
        }
    }

    sum
}

fn vec_sum2(data: &[String], usable_index: &[usize], offset: usize) -> usize
{
    let mut sum = 0;

    for index in usable_index {
        let el = data.get(*index).unwrap().chars().nth(offset).unwrap();
        if el == '1' {
            sum += 1;
        }
    }

    sum
}

fn part1(data: &[String]) -> u32
{
    let sum = vec_sum(data);

    let mut gamma_bin = String::new();
    let mut epsilon_bin = String::new();

    for el in sum {
        if el > data.len() / 2 {
            gamma_bin += "1";
            epsilon_bin += "0";
        }
        else {
            gamma_bin += "0";
            epsilon_bin += "1";
        }
    }

    let gamma = u32::from_str_radix(gamma_bin.as_str(), 2).unwrap();
    let epsilon = u32::from_str_radix(epsilon_bin.as_str(), 2).unwrap();

    gamma * epsilon
}

fn part2(data: &[String], criteria: usize) -> u32
{
    let mut usable_index: Vec<usize> = (0..data.len()).collect();

    let crit1: char;
    let crit2: char;

    if criteria == 1 {
        crit1 = '1';
        crit2 = '0';
    }
    else {
        crit1 = '0';
        crit2 = '1';
    }

    let mut oxygen_index = 0;

    for i in 0..data[0].len() {
        let sum = vec_sum2(data, &usable_index, i);
        let mut common_value = if sum > usable_index.len() / 2 {crit1} else {crit2};
        if usable_index.len() == sum*2 {
            common_value = crit1;
        }

        usable_index.retain(|&x| data[x].chars().nth(i).unwrap() == common_value);

        if usable_index.len() == 1 {
            oxygen_index = usable_index[0];
            break;
        }
    }

    u32::from_str_radix(data[oxygen_index].as_str(), 2).unwrap()
}

fn main()
{
    let data = utils::read_input_file();

    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data, 1) * part2(&data, 0));
}
