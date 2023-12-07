use std::{num::ParseIntError, str::FromStr};

struct RangedPoint {
    source: u64,
    destination: u64,
    range: u64,
}

impl FromStr for RangedPoint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_ascii_whitespace()
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map(|v| RangedPoint {
                destination: v[0],
                source: v[1],
                range: v[2],
            })
    }
}

impl RangedPoint {
    fn contains(&self, point: &u64) -> bool {
        let r = self.source..=self.source + self.range;
        r.contains(point)
    }
}

struct RangedMap {
    points: Vec<RangedPoint>,
}

impl FromStr for RangedMap {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RangedMap {
            points: s
                .lines()
                .skip(1)
                .map(|l| l.parse::<RangedPoint>().unwrap())
                .collect(),
        })
    }
}

impl RangedMap {
    fn get(&self, point: &u64) -> u64 {
        match self
            .points
            .iter()
            .filter(|p| p.contains(point))
            .collect::<Vec<_>>()
            .first()
        {
            None => point.clone(),
            Some(pt) => pt.destination + (point - pt.source),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input05.txt").unwrap();
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let mut destinations: Vec<u64> = seeds
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    let sectors: Vec<&str> = maps.split("\n\n").collect();
    let sectors: Vec<RangedMap> = sectors.iter().map(|s| s.parse().unwrap()).collect();

    for sector in sectors {
        for (index, dest) in destinations.clone().iter().enumerate() {
            let point = sector.get(dest);
            destinations.remove(index);
            destinations.insert(index, point);
        }
    }

    println!("{:?}", destinations.iter().min());
}
