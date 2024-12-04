use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "src/file.txt";
    let mut safe_reports = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(numbers) = line {
                let levels: Vec<i32> = numbers
                    .split_whitespace()
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect();
                if is_safe_report(&levels) {
                    safe_reports += 1;
                }
            }
        }
    }

    println!("Number of safe reports: {safe_reports}");
    Ok(())
}

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let mut direction: Option<i32> = None;
    for pair in levels.windows(2) {
        let diff = pair[1] - pair[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        let current_direction = diff.signum();
        if let Some(prev_direction) = direction {
            if prev_direction != current_direction {
                return false;
            }
        } else {
            direction = Some(current_direction);
        }
    }

    true
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
