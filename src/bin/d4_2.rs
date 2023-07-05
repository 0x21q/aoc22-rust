use std::{io, collections::HashSet};

fn main() -> io::Result<()> {
    let mut counter: u32 = 0;
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().is_empty() {
            break;
        }

        let nums: Vec<i32> = input
            .split(|c| c == '-' || c == ',')
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let count = (nums[0]..=nums[1])
            .collect::<HashSet<_>>()
            .intersection(&(nums[2]..=nums[3])
                .collect::<HashSet<_>>())
            .count();

        if count > 0 {
            counter += 1;
        }
    }
    println!("The result is {}", counter);
    Ok(())
}