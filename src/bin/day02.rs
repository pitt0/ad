#[derive(Debug)]
struct Round {
    red_cubes: u8,
    green_cubes: u8,
    blue_cubes: u8
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>
}

fn find_cubes_n(round: &str, color: &str) -> u8 {
    match round.split(", ").find(|s| { s.contains(color) }) {
        None => 0,
        Some(string) => string.chars().rev().skip(color.len() + 1).collect::<String>().chars().rev().collect::<String>().parse().unwrap()
    }
}

fn form_games(line: &str) -> Game {
    let game_data = line.split(": ").collect::<Vec<&str>>();
    let game_id = game_data[0].chars().skip(5).collect::<String>().parse::<u32>().unwrap();

    let rounds = game_data[1].split("; ").collect::<Vec<&str>>();
    let mut rounds_vec: Vec<Round> = Vec::new();

    for round in rounds {
        rounds_vec.push(
            Round {
                red_cubes: find_cubes_n(round, "red"),
                green_cubes: find_cubes_n(round, "green"),
                blue_cubes: find_cubes_n(round, "blue"),
            }
        )
    }

    Game {
        id: game_id,
        rounds: rounds_vec
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input02.txt").unwrap();
    let sum = input
        .lines()
        .map(form_games)
        .filter(|g| {
            match g.rounds
                .iter()
                .find(|r| { r.red_cubes > 12 || r.green_cubes > 13 || r.blue_cubes > 14 }) {
                    None => true,
                    _ => false
                }
        })
        .map(|g| {
            g.id
        }).sum::<u32>();
    println!("{sum}");
}