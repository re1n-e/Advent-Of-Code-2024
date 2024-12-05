use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "src/file.txt";
    let mut ans = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                let v: Vec<char> = l.chars().collect();
                ans += match_mul(v);
            }
        }
    }

    println!("{ans}");
    Ok(())
}

fn calculate_mul(mul_string: &str) -> i32 {
    let mul: Vec<&str> = mul_string.split(",").collect();
    if mul.len() != 2 {
        return 0;
    }
    mul[0].parse::<i32>().unwrap_or(0) * mul[1].parse::<i32>().unwrap_or(0)
}

fn match_mul(v: Vec<char>) -> i32 {
    let mut res = 0;
    let mut i = 0;
    while i < v.len() - 4 {
        let mut inside = false;
        if v[i] == 'm' && v[i + 1] == 'u' && v[i + 2] == 'l' && v[i + 3] == '(' {
            inside = true;
            let mut j = i + 4;
            let mut inside_bracket = String::new();
            while j < v.len() {
                if v[j] == ')' {
                    inside = false;
                    res += calculate_mul(&inside_bracket);
                    i = j;
                }
                if !v[j].is_digit(10) && v[j] != ',' {
                    break;
                }
                inside_bracket.push(v[j]);
                j += 1;
            }
        }
        if inside {
            i = i + 4;
        } else {
            i += 1;
        }
    }
    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
