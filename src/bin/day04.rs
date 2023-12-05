// struct Card {
//     id: u8,
//     numbers: Vec<u8>,
// }

// fn get_id(game_name: &str) -> u8 {
//     game_name
//         .chars()
//         .filter(|c| c.is_ascii_digit())
//         .collect::<String>()
//         .parse()
//         .unwrap()
// }

fn get_winning_numbers(numbers: &str) -> Vec<u8> {
    let cat: Vec<&str> = numbers.split('|').collect();
    let wins: Vec<&str> = cat.first().unwrap().split(' ').collect();
    let nums: Vec<&str> = cat.last().unwrap().split(' ').collect();
    let mut numbers: Vec<u8> = Vec::new();

    for win in wins {
        if win.is_empty() {
            continue;
        }
        if nums.contains(&win) {
            numbers.push(win.parse().unwrap());
        }
    }
    numbers
}

fn get_value(line: &str) -> u32 {
    let game: Vec<&str> = line.split(':').collect();
    // let card = Card {
    //     id: get_id(game.first().unwrap()),
    //     numbers: get_winning_numbers(game.first().unwrap()),
    // };

    let mut val: u32 = 0;
    for _ in get_winning_numbers(game.last().unwrap()) {
        if val == 0 {
            val = 1;
        } else {
            val *= 2;
        }
    }

    val
}

fn main() {
    let input = std::fs::read_to_string("src/input04.txt").unwrap();
    let sum: u32 = input.lines().map(get_value).sum();
    println!("{sum}");
}
