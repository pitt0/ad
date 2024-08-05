use itertools::Itertools;
use std::{cmp::Ordering, fs};

fn flatten_hand(score: &str) -> itertools::Unique<std::str::Chars> {
    score.chars().unique()
}

// score formula:
// suqared(smallest number > 1) * the rest of the Vec
fn get_hand_score(hand: &str) -> u32 {
    let mut cards_scores: Vec<u32> = Vec::new();

    for card in flatten_hand(hand) {
        let num = hand.chars().filter(|c| *c == card).count() as u32;
        cards_scores.push(num);
    }

    let sec_smallest = match cards_scores.iter().filter(|s| *s > &1).min() {
        None => 1,
        Some(val) => *val,
    };

    let mut score = sec_smallest;
    for s in cards_scores {
        score *= s
    }
    score
}

fn get_power(card: char) -> usize {
    let mut values = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    // I copied it this way and don't want to edit it manyally
    values.reverse();
    values.iter().position(|&c| c == card).unwrap()
}

fn compare_card_no(a: char, b: char) -> Ordering {
    let a_pow = get_power(a);
    let b_pow = get_power(b);
    if a_pow > b_pow {
        return Ordering::Greater;
    } else if b_pow > a_pow {
        return Ordering::Less;
    }

    Ordering::Equal
}

fn get_card_at(hand: &str, pos: usize) -> char {
    hand.chars().nth(pos).unwrap()
}

fn compare_scores(a: &(&str, u32, u64), b: &(&str, u32, u64)) -> Ordering {
    if a.1 == b.1 {
        for i in 0..5 {
            let comp = compare_card_no(get_card_at(a.0, i), get_card_at(b.0, i));
            match comp {
                Ordering::Equal => {
                    continue;
                }
                _ => {
                    return comp;
                }
            }
        }
        return Ordering::Equal;
    }

    if a.1 > b.1 {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

fn main() {
    let content = fs::read_to_string("tests/day7.txt").unwrap();

    let mut scores: Vec<(&str, u32, u64)> = Vec::new();
    for (hand, bid) in content
        .lines()
        .map(|hb| hb.split(" ").next_tuple().unwrap())
    {
        scores.push((hand, get_hand_score(hand), bid.parse().unwrap()))
    }

    scores.sort_by(compare_scores);
    let mut x = scores.iter().enumerate().map(|(i, (hand, score, bid))| {
        let mut result = String::new();
        result.push_str(hand);
        result.push_str(" ->  ");
        result.push_str(&score.to_string());
        result.push_str(" ");
        result.push_str(&(bid * ((i as u64) + 1)).to_string());
        result
    });

    let _ = fs::write("results/day7.txt", x.join("\n"));

    let res = scores
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * ((i as u64) + 1))
        .sum::<u64>();

    println!("Result is: {}", res);
}
