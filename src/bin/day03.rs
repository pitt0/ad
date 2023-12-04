#[derive(Debug, Clone)]
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

    // fn is_symbol(&self, pos: &Position) -> bool {
    //     let c = self.get_char(pos);
    //     match c {
    //         None => false,
    //         Some(c) => !c.is_ascii_digit() && c != '.',
    //     }
    // }

    fn is_number(&self, pos: &Position) -> bool {
        let c = self.get_char(pos);
        match c {
            None => false,
            Some(c) => c.is_ascii_digit(),
        }
    }
}

fn get_full_number(matrix: &Matrix, position: &Position) -> String {
    let mut number = String::new();
    let mut pos = position.clone();
    loop {
        pos = match pos.clone().try_add((-1, 0)) {
            None => break,
            Some(p) => p,
        };
        match matrix.get_char(&pos) {
            None => break,
            Some(c) => {
                if !c.is_ascii_digit() {
                    pos = pos.try_add((1, 0)).unwrap();
                    break;
                }
            }
        };
    }
    for c in matrix.get_line(&pos).unwrap().chars().skip(pos.x) {
        match c.to_digit(10) {
            None => break,
            Some(_) => number.push(c),
        }
    }
    number
}

// fn has_been_checked(matrix: &Matrix, position: &Position) -> bool {
//     if position.x == 0 {
//         return false;
//     }

//     match matrix.get_char(position) {
//         None => false,
//         Some(c) => {
//             if c.is_ascii_digit() {
//                 return true;
//             }
//             false
//         }
//     }
// }

// fn number_found(matrix: &Matrix, position: Position) -> Option<u32> {
//     if has_been_checked(matrix, &position) {
//         return None;
//     }

//     let number = get_full_number(matrix, &position);
//     const ADJACENTS: [(i32, i32); 8] = [
//         (-1, -1),
//         (0, -1),
//         (1, -1),
//         (-1, 0),
//         (1, 0),
//         (-1, 1),
//         (0, 1),
//         (1, 1),
//     ];

//     let mut pos = position.clone();
//     for _ in number.chars() {
//         for p in ADJACENTS {
//             match pos.clone().try_add(p) {
//                 None => {}
//                 Some(pos) => {
//                     if matrix.is_symbol(&pos) {
//                         return Some(number.parse().unwrap());
//                     }
//                 }
//             }
//         }
//         pos = match pos.try_add((1, 0)) {
//             None => break,
//             Some(val) => val,
//         }
//     }
//     None
// }

fn star_found(matrix: &Matrix, position: Position) -> Option<u32> {
    let mut n_found: u8 = 0;
    let mut numbers: Vec<u32> = Vec::new();
    let mut found: bool;

    let mut p: (i32, i32);

    for y in -1..=1 {
        found = false;
        for x in -1..=1 {
            p = (x, y);
            match position.clone().try_add(p) {
                None => {}
                Some(pos) => {
                    if !matrix.is_number(&pos) {
                        found = false;
                        continue;
                    }
                    if matrix.is_number(&pos) && !found {
                        found = true;
                        n_found += 1;
                        numbers.push(get_full_number(matrix, &pos).parse().unwrap());
                    }
                }
            }
        }
        if n_found == 2 {
            let n1 = numbers.first().unwrap();
            let n2 = numbers.last().unwrap();
            // if n1 == &848u32 || n2 == &848u32 {
            //     println!("Found on line {}", position.y);
            // }
            let r = n1 * n2;
            // println!("{n1}*{n2}={r}");
            return Some(r);
        }
    }
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
                if c == '*' {
                    match star_found(&matrix, Position { x, y }) {
                        None => continue,
                        Some(num) => numbers.push(num),
                    }
                }
            }
            if numbers.is_empty() {
                return 0;
            }
            println!("Summing {} numbers for line {y}.", numbers.len());
            let sum = numbers.iter().sum::<u32>();
            println!("Sum is {sum}");
            numbers.iter().sum::<u32>()
        })
        .sum::<u32>();

    println!("{s}");
}
