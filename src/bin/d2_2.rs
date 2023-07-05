use std::io;

#[derive(Copy, Clone)]
enum GameMove {
    Rock,
    Paper,
    Scissors
}

fn get_score(player_move: &GameMove, enemy_move: &GameMove) -> u32 {
    let score = match (player_move, enemy_move) {
        (GameMove::Rock, GameMove::Scissors) => 6 + 1,
        (GameMove::Paper, GameMove::Rock) => 6 + 2,
        (GameMove::Scissors, GameMove::Paper) => 6 + 3,
        (GameMove::Rock, GameMove::Rock) => 3 + 1,
        (GameMove::Paper, GameMove::Paper) => 3 + 2,
        (GameMove::Scissors, GameMove::Scissors) => 3 + 3,
        (GameMove::Rock, GameMove::Paper) => 0 + 1,
        (GameMove::Paper, GameMove::Scissors) => 0 + 2,
        (GameMove::Scissors, GameMove::Rock) => 0 + 3,
    };
    score
}

fn parse_move(input: &str) -> GameMove {
    match input {
        "A" => GameMove::Rock,
        "B" => GameMove::Paper,
        "C" => GameMove::Scissors,
        _ => panic!("Invalid move")
    }
}

fn calculate_move(input: &str, enemy: GameMove) -> GameMove {
    match input {
        "X" => lose(enemy),
        "Y" => enemy,
        "Z" => win(enemy),
        _ => panic!("Invalid move")
    }
}

fn win(enemy: GameMove) -> GameMove {
    match enemy {
        GameMove::Rock => GameMove::Paper,
        GameMove::Paper => GameMove::Scissors,
        GameMove::Scissors => GameMove::Rock,
    }
}

fn lose(enemy: GameMove) -> GameMove {
    match enemy {
        GameMove::Rock => GameMove::Scissors,
        GameMove::Paper => GameMove::Rock,
        GameMove::Scissors => GameMove::Paper,
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
        let enemy_move = parse_move(splitted[0]);
        let player_move = calculate_move(splitted[1], enemy_move);
        sum += get_score(&player_move, &enemy_move);
    }
    println!("The result is {}", sum);
    Ok(())
}