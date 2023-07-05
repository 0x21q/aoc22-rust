use std::io;

enum GameMove {
    Rock,
    Paper,
    Scissors
}

// returns score
fn play_game(player_move: GameMove, enemy_move: GameMove) -> u32 {
    match (player_move, enemy_move) {
        (GameMove::Rock, GameMove::Scissors) => 6 + 1,
        (GameMove::Paper, GameMove::Rock) => 6 + 2,
        (GameMove::Scissors, GameMove::Paper) => 6 + 3,
        (GameMove::Rock, GameMove::Rock) => 3 + 1,
        (GameMove::Paper, GameMove::Paper) => 3 + 2,
        (GameMove::Scissors, GameMove::Scissors) => 3 + 3,
        (GameMove::Rock, GameMove::Paper) => 0 + 1,
        (GameMove::Paper, GameMove::Scissors) => 0 + 2,
        (GameMove::Scissors, GameMove::Rock) => 0 + 3,
    }
}

fn parse_move(input: &str) -> GameMove {
    match input {
        "A" | "X" => GameMove::Rock,
        "B" | "Y" => GameMove::Paper,
        "C" | "Z" => GameMove::Scissors,
        _ => panic!("Invalid move")
    }
}

fn main() -> io::Result<()> {
    let mut sum: u32 = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().is_empty() {
            break;
        }

        let splitted: Vec<&str> = input.trim().split_whitespace().collect();
        sum += play_game(parse_move(splitted[1]), parse_move(splitted[0]));
    }
    println!("The result is {}", sum);
    Ok(())
}

