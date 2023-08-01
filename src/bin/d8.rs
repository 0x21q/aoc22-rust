use std::io;

fn load_stdin() -> Result<String, io::Error> {
    let mut input: String = String::new();
    loop {
        let mut line: String = String::new();
        io::stdin().read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }
        input.push_str(&line);
    }
    Ok(input)
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let mut forest: Vec<Vec<char>> = Vec::new(); 
    for (i, line) in input.lines().enumerate() {
        forest.push(Vec::new());
        for c in line.chars() {
            forest[i].push(c);
        }
    }
    forest
}

fn is_visible(forest: &Vec<Vec<char>>, i: &usize, j: &usize) -> bool {
    return visible_any(forest, i, j, &(-1,0))
        || visible_any(forest, i, j, &(1,0))
        || visible_any(forest, i, j, &(0,-1))
        || visible_any(forest, i, j, &(0,1));
}

fn calc_distance(forest: &Vec<Vec<char>>, i: &usize, j: &usize) -> usize {
    return calc_any(forest, i, j, &(-1,0))
        * calc_any(forest, i, j, &(1,0))
        * calc_any(forest, i, j, &(0,-1))
        * calc_any(forest, i, j, &(0,1));
}

fn visible_any(forest: &Vec<Vec<char>>, i: &usize, j: &usize, d: &(i8, i8)) -> bool {
    let tree_h = forest[*i][*j];
    let mut i = *i as i8 + d.0;
    let mut j = *j as i8 + d.1;

    while let Some(temp_h) = forest.get(i as usize).and_then(|row| row.get(j as usize)) {
        if *temp_h >= tree_h {
            return false;
        }
        i += d.0;
        j += d.1;
    }
    return true;
}

fn calc_any(forest: &Vec<Vec<char>>, i: &usize, j: &usize, d: &(i8, i8)) -> usize {
    let tree_h = forest[*i][*j];
    let mut i = *i as i8 + d.0;
    let mut j = *j as i8 + d.1;
    let mut result = 0;

    while let Some(temp_h) = forest.get(i as usize).and_then(|row| row.get(j as usize)) {
        result += 1;
        if *temp_h >= tree_h {
            break;
        }
        i += d.0;
        j += d.1;
    }
    return result;
}

fn solve1(forest: &Vec<Vec<char>>) -> usize {
    let mut result = forest[0].len()*2 + forest.len()*2 - 4;
    for i in 1..forest.len()-1 {
        for j in 1..forest[0].len()-1 {
            if is_visible(forest, &i, &j) {
                result += 1;
            }
        }
    }
    result
}

fn solve2(forest: &Vec<Vec<char>>) -> usize {
    let mut results: Vec<usize> = Vec::new();
    for i in 1..forest.len()-1 {
        for j in 1..forest[0].len()-1 {
            results.push(calc_distance(forest, &i, &j));
        }
    }
    results.sort();
    return results[results.len()-1];
}

fn main() -> io::Result<()> {
    let input = load_stdin()?;
    let forest = parse_input(input);
    println!("The result1 is {}", solve1(&forest));
    println!("The result2 is {}", solve2(&forest));
    Ok(())
}