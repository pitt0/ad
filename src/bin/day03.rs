#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn try_add(self, pos: (i32, i32)) -> Option<Self> {
        let x = if pos.0.is_negative() {
            if self.x < pos.0.wrapping_abs() as u32 as usize {
                return None;
            }
            self.x - pos.0.wrapping_abs() as u32 as usize
        } else {
            self.x + pos.0 as usize
        };
        let y = if pos.1.is_negative() {
            if self.y < pos.1.wrapping_abs() as u32 as usize {
                return None;
            }
            self.y - pos.1.wrapping_abs() as u32 as usize
        } else {
            self.y + pos.1 as usize
        };

        Some(Position { x, y })
    }
}

struct Matrix<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Matrix<'a> {
    fn new() -> Self {
        Matrix { lines: Vec::new() }
    }

    fn push(&mut self, line: &'a str) -> &Self {
        self.lines.push(line);
        self
    }

    fn get_char(&self, pos: &Position) -> Option<char> {
        if pos.y > self.lines.len() {
            return None;
        }
        if pos.x > self.lines[pos.y].len() {
            return None;
        }
        self.lines[pos.y].chars().nth(pos.x)
    }

    fn get_line(&self, pos: &Position) -> Option<&str> {
        if pos.y > self.lines.len() {
            return None;
        }
        Some(self.lines[pos.y])
    }

    fn is_symbol(&self, pos: &Position) -> bool {
        let c = self.get_char(pos);
        match c {
            None => false,
            Some(c) => !c.is_digit(10) && c != '.',
        }
    }
}

fn get_full_number(line: &str, position: &Position) -> String {
    let mut number = String::new();
    for c in line.chars().skip(position.x) {
        match c.to_digit(10) {
            None => break,
            Some(_) => number.push(c),
        }
    }
    number
}

fn number_found(matrix: &Matrix, position: Position) -> Option<u32> {
    // check if the char before this is a number aswell
    // if it is, it skips, the function returns None
    match position.clone().try_add((-1, 0)) {
        None => {}
        Some(pos) => match matrix.get_char(&pos) {
            None => {}
            Some(c) => {
                if c.is_digit(10) {
                    return None;
                }
            }
        },
    };

    let number = get_full_number(matrix.get_line(&position).unwrap(), &position);
    const ADJACENTS: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut pos = position.clone();
    for _ in number.chars() {
        for p in ADJACENTS {
            match pos.clone().try_add(p) {
                None => {}
                Some(pos) => {
                    if matrix.is_symbol(&pos) {
                        println!("Number {number} is being added.");
                        return Some(number.parse().unwrap());
                    }
                }
            }
        }
        pos = match pos.try_add((1, 0)) {
            None => break,
            Some(val) => val,
        }
    }
    println!("Number {number} is being skipped.");
    None
}

fn main() {
    let input = std::fs::read_to_string("src/input03.txt").unwrap();
    let mut matrix = Matrix::new();
    for line in input.split('\n') {
        matrix.push(line);
    }
    let s = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            let mut numbers: Vec<u32> = Vec::new();
            for (x, c) in l.chars().enumerate() {
                if c.is_digit(10) {
                    match number_found(&matrix, Position { x, y }) {
                        None => continue,
                        Some(num) => numbers.push(num),
                    }
                }
            }
            numbers.iter().sum::<u32>()
        })
        .sum::<u32>();

    println!("{s}");
}
