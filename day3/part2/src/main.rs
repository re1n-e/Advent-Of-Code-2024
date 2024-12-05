use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "src/file.txt";
    let mut ans = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                ans += process_line(&l);
            }
        }
    }

    println!("{ans}");
    Ok(())
}

/// Processes a line of corrupted memory, extracting `mul()` instructions and applying `do()`/`don't()` rules.
fn process_line(line: &str) -> i32 {
    let mut res = 0;
    let mut do_dont = true; // Initially, `mul` instructions are enabled.
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == 'm' {
            // Handle `mul()` or invalid patterns starting with 'm'
            if chars.peek() == Some(&'u') && chars.nth(1) == Some('l') && chars.next() == Some('(') {
                let mut inside = String::new();
                while let Some(&next) = chars.peek() {
                    if next == ')' {
                        chars.next(); // Consume ')'
                        break;
                    }
                    if next.is_digit(10) || next == ',' {
                        inside.push(chars.next().unwrap());
                    } else {
                        chars.next(); // Skip invalid character
                    }
                }

                // Calculate and add only if `do_dont` is true
                if do_dont {
                    res += calculate_mul(&inside);
                }
            }
        } else if c == 'd' {
            // Handle `do()` and `don't()` instructions
            let next_chars: String = chars.clone().take(6).collect();
            if next_chars.starts_with("o()") {
                chars.nth(2); // Consume "o()"
                do_dont = true;
            } else if next_chars.starts_with("on't()") {
                chars.nth(5); // Consume "on't()"
                do_dont = false;
            }
        }
    }

    res
}

/// Calculate multiplication from a comma-separated string like "3,4".
fn calculate_mul(mul_string: &str) -> i32 {
    let parts: Vec<&str> = mul_string.split(',').collect();
    if parts.len() != 2 {
        return 0; // Invalid format
    }

    let num1 = parts[0].trim().parse::<i32>().unwrap_or(0);
    let num2 = parts[1].trim().parse::<i32>().unwrap_or(0);
    num1 * num2
}

/// Read lines from a file and return an iterator.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
