use std::io;

fn parse_cranes(line: &str, stacks: &mut Vec<Vec<char>>) {
    let mut stck_cnt: usize = 0;
    let mut ws_cnt: usize = 0;

    for c in line.trim().chars() {
        match c {
            '[' | ']' => continue,
            ' ' => {
                if ws_cnt == 4 {
                    stck_cnt += 1;
                    ws_cnt = 0;
                }
                ws_cnt += 1;
            }
            _ => {
                stacks[stck_cnt].push(c);
                stck_cnt += 1;
                ws_cnt = 0;
            }
        }
    }
}

fn parse_moves(line: &str, stacks: &mut Vec<Vec<char>>) {
    let splitted: Vec<&str> = line.trim().split_whitespace().collect();
    let n_cranes: usize = splitted[1].parse().unwrap();
    let from: usize = splitted[3].parse().unwrap();
    let to: usize = splitted[5].parse().unwrap();

    let mut cargo: Vec<char> = stacks[from-1].drain(0..n_cranes).collect();
    cargo.append(&mut stacks[to-1]);
    stacks[to-1] = cargo;        
}

fn get_top_cranes(stacks: &mut Vec<Vec<char>>) -> String {
    let mut top_cranes: String = String::new();
    for stack in stacks {
        if stack.len() != 0 {
            stack.iter().find(|&c| *c != ' ').map(|c| top_cranes.push(*c));
        }
    }
    top_cranes
}

fn main() -> io::Result<()> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        if line.trim().eq("end") {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }

        if !line.trim().contains("move") {
            parse_cranes(&line, &mut stacks);
        } else {
            parse_moves(&line, &mut stacks);
        }
    }
    println!("{}", get_top_cranes(&mut stacks));
    Ok(())
}