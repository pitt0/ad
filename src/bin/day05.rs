fn main() {
    let input = std::fs::read_to_string("src/input05.txt").unwrap();
    let sectors: Vec<&str> = input.split("\n\n").collect();

    sectors.iter().skip(1).map(|s| s.lines().skip(1));

    println!("{:?}", sectors);
}
