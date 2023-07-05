use std::io;

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

        if nums[0] >= nums[2] && nums[1] <= nums[3] {
            counter += 1;
        } else if nums[2] >= nums[0] && nums[3] <= nums[1] {
            counter += 1;
        }
    }
    println!("The result is {}", counter);
    Ok(())
}