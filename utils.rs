use std::io::BufRead;

fn get_input_filename() -> String
{
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Missing input file.");
        std::process::exit(1);
    }

    args[1].clone()
}

pub fn read_input_file() -> Vec<String>
{
    let filename = get_input_filename();

    let file = std::fs::File::open(filename).expect("No such file.");
    let buf = std::io::BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
