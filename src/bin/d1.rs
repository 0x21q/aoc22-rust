use std::io;

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    loop {
        let mut line: String = String::new();
        io::stdin().read_line(&mut line)?;
        if line.starts_with(".") {
            break;
        }
        input.push_str(&line);
    }

    let mut cals: Vec<usize> = input
        .split("\n\n")
        .map(|elf| {
            elf
                .split("\n")
                .map(|food| food.parse::<usize>().unwrap_or(0))
                .sum()
        })
        .collect();

    cals.sort();
    println!("The result one is {}", cals[cals.len() - 1]);
    println!("The result two is {}", cals[cals.len() - 1] + cals[cals.len() - 2] + cals[cals.len() - 3]);
    Ok(())
}