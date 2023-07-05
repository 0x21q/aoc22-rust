use std::io;
use std::collections::HashSet;

fn load_stdin() -> Result<String, io::Error> {
    let mut input: String = String::new();
    loop {
        let mut line: String = String::new();
        io::stdin().read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }
        input.push_str(&line.trim());
    }
    Ok(input)
}

fn solve(input: &str, win: usize) -> usize {
    let mut i: usize = 0;
    while let Some(w) = input.get(i..i+win) {
        let hset: HashSet<char> = HashSet::from_iter(w.chars());
        if hset.len() == win {
            break;
        }
        i += 1;
    }
    i+win
}

fn main() -> io::Result<()> {
    let input: String = load_stdin()?;
    println!("The result one is {}", solve(&input, 4));
    println!("The result two is {}", solve(&input, 14));
    Ok(())
}