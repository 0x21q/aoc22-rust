use std::io;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut sum = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().is_empty() {
            break;
        }

        let mid = input.len() / 2;
        let compart1: HashSet<char> = input.chars().take(mid).collect();
        let compart2: HashSet<char> = input.chars().skip(mid).take(input.len() - mid - 1).collect();

        if let Some(ch) = compart1.intersection(&compart2).next() {
            sum += get_priority(*ch);
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