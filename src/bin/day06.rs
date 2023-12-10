fn main() {
    let input = std::fs::read_to_string("src/input06.txt").unwrap();

    let (times, distances) = input.split_once('\n').unwrap();

    let time: f64 = times
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: f64 = distances
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    let min = ((time - (time * time - 4f64 * distance).sqrt()) / 2f64).ceil() as u64;
    let max = ((time + (time * time - 4f64 * distance).sqrt()) / 2f64).floor() as u64;

    let result = max - min + 1;

    // let times: Vec<u16> = times
    //     .split_ascii_whitespace()
    //     .skip(1)
    //     .map(|n| n.parse().unwrap())
    //     .collect();
    // let distances: Vec<u16> = distances
    //     .split_ascii_whitespace()
    //     .skip(1)
    //     .map(|n| n.parse().unwrap())
    //     .collect();

    // let mut wins: Vec<u64> = Vec::new();

    // for i in 0..times.len() {
    //     let x = *times.get(i).unwrap() as f64;
    //     let y = *distances.get(i).unwrap() as f64;

    //     let min = ((x - (x * x - 4f64 * y).sqrt()) / 2f64).ceil() as u64;
    //     let max = ((x + (x * x - 4f64 * y).sqrt()) / 2f64).floor() as u64;

    //     wins.push(max - min + 1);
    // }

    // let mut result: u64 = 1;
    // for win in wins.iter() {
    //     result *= win;
    // }
    println!("{result}"); // NOTE - 1543104 is too low
}
