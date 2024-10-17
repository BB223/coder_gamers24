use std::{
    collections::HashSet, fs::File, io::{self, BufRead}, path::Path
};

fn main() {
    if let Ok(lines) = read_lines("./4_input.txt") {
        let mut iter = lines.map_while(Result::ok);
        while let Some(line) = iter.next() {
            let input: HashSet<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

            let min = input.iter().min().unwrap();
            let min = min.clone();
            let max = input.iter().max().unwrap();
            let max = max.clone();
            let missing = max + 1 - min - input.len() as u32;
            print!("{}: ", missing);

            let mut missing = Vec::new();

            for val in min + 1..max {
                if !input.contains(&val) {
                    missing.push(val)
                }
            }

            missing.sort();
            let missing: Vec<String> = missing.iter().map(|i| i.to_string()).collect();
            println!("{}", missing.join(", "));
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
