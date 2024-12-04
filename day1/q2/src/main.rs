use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "src/file.txt";
    let mut vector1: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(numbers) = line {
                let parts: Vec<&str> = numbers.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                    {
                        vector1.push(num1);
                        *map.entry(num2).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    let mut res = 0;
    for v1 in vector1.iter() {
        if let Some(val) = map.get(v1) {
            res += val * v1;
        }
    }
    println!("{res}");
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
