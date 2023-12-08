#![allow(dead_code)]
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct RangedMono {
    start: i64,
    range: i64, // NOTE - range includes starting point
}

impl RangedMono {
    fn last(&self) -> i64 {
        self.start + self.range - 1
    }
}

#[derive(Debug)]
struct RangedPoint {
    source: i64,
    destination: i64,
    range: i64, // NOTE - range includes starting point
}

impl FromStr for RangedPoint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_ascii_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
            .map(|v| RangedPoint {
                destination: v[0],
                source: v[1],
                range: v[2],
            })
    }
}

impl RangedPoint {
    fn limit(&self) -> i64 {
        self.source + self.range
    }

    fn offset(&self) -> i64 {
        self.destination - self.source
    }

    fn contains(&self, point: &i64) -> bool {
        (self.source..self.limit()).contains(point)
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
    fn get(&self, point: &i64) -> i64 {
        match self.points.iter().find(|&p| p.contains(point)) {
            None => *point,
            Some(pt) => point + pt.offset(),
        }
    }

    fn add_to_cache(cache: Option<Vec<RangedMono>>, mono: RangedMono) -> Vec<RangedMono> {
        match cache {
            None => Vec::from([mono]),
            Some(mut c) => {
                c.push(mono);
                c
            }
        }
    }

    fn get_range(&self, mono: &RangedMono, cache: Option<Vec<RangedMono>>) -> Vec<RangedMono> {
        match self.points.iter().find(|&p| p.contains(&mono.start)) {
            None => match self.points.iter().find(|&p| p.contains(&mono.last())) {
                None => RangedMap::add_to_cache(cache, *mono),
                Some(point) => {
                    let offset = mono.last() - point.source;
                    let not_found = RangedMono {
                        start: mono.start,
                        range: mono.range - offset,
                    };
                    let rest = RangedMono {
                        start: point.source,
                        range: offset,
                    };
                    self.get_range(&rest, Some(RangedMap::add_to_cache(cache, not_found)))
                }
            },
            Some(point) => {
                if !point.contains(&mono.last()) {
                    let offset = point.limit() - mono.start;
                    let rest = RangedMono {
                        start: mono.start + offset,
                        range: mono.range - offset,
                    };
                    let destination = RangedMono {
                        start: mono.start + point.offset(),
                        range: offset,
                    };
                    self.get_range(&rest, Some(RangedMap::add_to_cache(cache, destination)))
                } else {
                    RangedMap::add_to_cache(
                        cache,
                        RangedMono {
                            start: mono.start + point.offset(),
                            range: mono.range,
                        },
                    )
                }
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input05.txt").unwrap();
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let mut destinations: Vec<RangedMono> = Vec::new();
    let mut seed: i64 = 0;
    for (idx, num) in seeds.split_ascii_whitespace().enumerate().skip(1) {
        if idx % 2 == 0 {
            destinations.push(RangedMono {
                start: seed,
                range: num.parse().unwrap(),
            })
        } else {
            seed = num.parse().unwrap();
        }
    }
    // let mut destinations: Vec<i64> = seeds
    //     .split_ascii_whitespace()
    //     .skip(1)
    //     .map(|n| n.parse().unwrap())
    //     .collect();

    let sectors: Vec<RangedMap> = maps.split("\n\n").map(|s| s.parse().unwrap()).collect();

    for sector in sectors.iter() {
        for dest in destinations.clone().iter() {
            let mut monos = sector.get_range(dest, None);
            destinations.remove(0);
            destinations.append(&mut monos);
        }
    }

    let mut min: i64 = i64::MAX;
    for num in destinations.iter() {
        if num.start < min {
            min = num.start;
        }
    }

    println!("{min}");
}
