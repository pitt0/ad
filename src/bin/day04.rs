struct Card {
    _id: u8,
    numbers: Vec<u8>,
    winning: Vec<u8>,
}

impl Card {
    fn get_matches(&self) -> Vec<&u8> {
        self.winning
            .iter()
            .filter(|win| self.numbers.contains(win))
            .collect()
    }
}

fn get_id(game_name: &str) -> u8 {
    game_name
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn get_numbers_vec(s: &str) -> Vec<u8> {
    s.split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_card(line: &str) -> Card {
    let game: Vec<&str> = line.split(':').collect();
    let cat: Vec<&str> = game.last().unwrap().split('|').collect();
    Card {
        _id: get_id(game.first().unwrap()),
        numbers: get_numbers_vec(cat.last().unwrap()),
        winning: get_numbers_vec(cat.first().unwrap()),
    }
}

// fn get_value(line: &str) -> u32 {
//     let card: Card = get_card(line);

//     let mut val: u32 = 0;
//     for _ in card.get_matches() {
//         if val == 0 {
//             val = 1;
//         } else {
//             val *= 2;
//         }
//     }

//     val
// }

fn main() {
    let input = std::fs::read_to_string("src/input04.txt").unwrap();

    let cards: Vec<Card> = input.lines().map(get_card).collect();

    let mut wins: Vec<u32> = vec![1; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        for w in 0..card.get_matches().len() {
            // if wins.get(index + w + 1).is_none() {
            //     wins[index + w + 1] = 0
            // }
            let to_add = wins.get(index).unwrap().clone();
            wins[index + w + 1] += to_add;
        }
    }
    println!("{}", wins.iter().sum::<u32>());
}
