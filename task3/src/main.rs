use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    // example input file
    // 3
    // 1 0 1 2 0 1
    // 2 0 2 0 1 1
    // 3
    // 1 0 2 0 1 1
    // 2 0 2 1 0 2
    if let Ok(lines) = read_lines("./3_input.txt") {
        let mut iter = lines.map_while(Result::ok);
        while let Some(line) = iter.next() {
            let faces: i32 = line.parse().unwrap();

            let line = iter.next().unwrap();
            let mut result: Vec<i32> = line
                .split(" ")
                .map(|stri| stri.parse::<i32>().unwrap())
                .collect();

            let line = iter.next().unwrap();
            let target: Vec<i32> = line
                .split(" ")
                .map(|stri| stri.parse::<i32>().unwrap())
                .collect();

            let mut res = Vec::new();

            for i in 0..result.len() {
                let rotation = target[i] - result[i];
                res.push(rotation.to_string());
                adder(&mut result, i, rotation, faces);
            }

            println!("[{}]", res.join(" "));
        }
    }
}

fn get_level(index: usize) -> usize {
    f64::floor((f64::sqrt(8.0 * index as f64 + 1.0) - 1.0) / 2.0) as usize
}

fn get_children_indices(index: usize) -> (usize, usize) {
    let lvl = get_level(index);
    (index + lvl + 1, index + lvl + 2)
}

fn adder(list: &mut Vec<i32>, index: usize, rotation: i32, faces: i32) {
    if list.get(index).is_none() {
        return;
    }
    let val = (list[index] + rotation) % faces;

    list[index] = val;

    let (left, right) = get_children_indices(index);

    adder(list, left, rotation, faces);
    adder(list, right, rotation, faces);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
