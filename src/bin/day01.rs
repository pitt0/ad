fn find_number(line: &str) -> u32 {
    // this does not work if a string has no number
    let d1 = line.chars().find(|c| c.is_numeric()).unwrap();
    let d2 = line.chars().rev().find(|c| c.is_numeric()).unwrap();
    format!("{d1}{d2}").parse().unwrap()
}

fn find_and_replace_number(line: &str) -> u32 {
    let line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .replace("zero", "z0o");
    find_number(&line)
}

fn main() {
    let input = std::fs::read_to_string("src/input01.txt").unwrap();
    let sum: u32 = input.lines().map(find_and_replace_number).sum();

    println!("{}", sum);
}
