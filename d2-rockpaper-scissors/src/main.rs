use core::fmt;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind, Result},
};

#[test]
fn test_small_score() {
    let score = read_encrypted_message_file("src/small-strategy.txt")
        .and_then(decrypt_all_moves)
        .map(get_total_score)
        .unwrap();
    assert_eq!(score, 15);
}

#[test]
fn test_score() {
    let score = read_encrypted_message_file("src/strategy.txt")
        .and_then(decrypt_all_moves)
        .map(get_total_score)
        .unwrap();
    assert_eq!(score, 14264);
}


#[test]
fn test_score2() {
    let moves: Vec<(Shape, Shape)> = read_encrypted_message_file("src/strategy.txt")
        .and_then(decrypt_all_moves)
        .unwrap()
        .into_iter()
        .map(decrypt_moveset2)
        .collect();

    assert_eq!(get_total_score(moves), 12382);
}


#[derive(Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Shape::{Paper, Rock, Scissors};
        let s = match self {
            Rock => "Rock",
            Paper => "Paper",
            Scissors => "Scissors",
        };
        format!("{}", s).fmt(f)
    }
}

fn shape_score(player_shape: &Shape) -> i32 {
    match player_shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn outcome_score(your_shape: &Shape, opponent_shape: &Shape) -> i32 {
    use Shape::{Paper, Rock, Scissors};

    match (your_shape, opponent_shape) {
        (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => 6,
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0,
    }
}

fn get_moveset_score((opponent_shape, your_shape): (Shape, Shape)) -> i32 {
    let x = outcome_score(&your_shape, &opponent_shape) + shape_score(&your_shape);
    return x;
}

fn read_encrypted_message_file(filename: &str) -> Result<Vec<(String, String)>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut predicted_moves: Vec<(String, String)> = Vec::new();

    let line_number = 0;
    for line in reader.lines() {
        let x = line?;
        let errfn = |s: &str| {
            Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("Error on line {}:\n{}\n{}", line_number, x, s),
            ))
        };
        if x.len() != 3 {
            return errfn("Length of line is not 3!");
        }

        let (opponent_shape, your_shape) = x.split_at(1);

        // Remove whitespace
        let a = opponent_shape.trim();
        let b = your_shape.trim();

        predicted_moves.push((a.to_string(), b.to_string()));
    }

    Ok(predicted_moves)
}

fn decrypt_moveset((opponent_shape, your_shape): (String, String)) -> Result<(Shape, Shape)> {
    use Shape::{Paper, Rock, Scissors};
    let errfn = |s: &str, e: &str| {
        Err(io::Error::new(
            ErrorKind::InvalidInput,
            format!("{}: {}", s, e),
        ))
    };

    // NOTE: Could make a parser struct to get line info
    let a = match opponent_shape.as_str() {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        s => return errfn(s, "Opponent Shape was not A,B, or C!"),
    };

    let b = match your_shape.as_str() {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        s => return errfn(s, "Your Shape was not X,Y, or Z!"),
    };

    Ok((a, b))
}

fn decrypt_moveset2((opponent_shape, your_shape): (Shape, Shape)) -> (Shape, Shape) {
    use Shape::{Paper, Rock, Scissors};
    match your_shape {
        // Draw
        Paper => (opponent_shape.clone(), opponent_shape),
        // Lose
        Rock => (opponent_shape.clone(),
                 match opponent_shape {
                     Rock => Scissors,
                     Paper => Rock,
                     Scissors => Paper,
                 }),
        // Win
        Scissors => (opponent_shape.clone(),
                 match opponent_shape {
                     Rock => Paper,
                     Paper => Scissors,
                     Scissors => Rock,
                 }),
    }
}


fn decrypt_all_moves(moves: Vec<(String, String)>) -> Result<Vec<(Shape, Shape)>> {
    moves.into_iter().map(decrypt_moveset).collect()
}

fn get_total_score(moves: Vec<(Shape, Shape)>) -> i32 {
    moves.into_iter().map(get_moveset_score).sum::<i32>()
}

fn main() {

    match read_encrypted_message_file("src/strategy.txt").and_then(decrypt_all_moves) {
        Err(e) => return println!("Error: {}", e.to_string()),
        Ok(moves) => println!("Total Score: {}", get_total_score(moves.into_iter().map(decrypt_moveset2).collect())),
    };
}
