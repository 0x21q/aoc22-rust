use std::io;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut sum = 0;
    'outer: loop {
        let mut input_vec: Vec<String> = Vec::new();
        for _ in 0..3 {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().is_empty() {
                break 'outer;
            }
            input_vec.push(input.trim().to_string());
        }

        let hash1: HashSet<char> = input_vec[0].chars().collect();
        let hash2: HashSet<char> = input_vec[1].chars().collect();

        if let Some(ch) = input_vec[2]
            .chars()
            .find(|ch| hash1.contains(ch) && hash2.contains(ch))
        {
            sum += get_priority(ch);
        }
    }
    println!("Sum is {}", sum);
    Ok(())
}

fn get_priority(ch: char) -> i32 {
    if ch.is_lowercase() {
        (ch as i32) - 96
    } else {
        (ch as i32) - 38
    }
}
