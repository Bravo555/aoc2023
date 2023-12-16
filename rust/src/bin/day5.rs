use std::{ops::Range, str::FromStr};

fn main() {
    let data = include_str!("../../../input/day5.txt");

    let (seeds_line, maps) = data.split_once("\n\n").unwrap();

    let seeds: Vec<u64> = seeds_line
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    let seed_to_location_map: LayeredMap = maps.parse().unwrap();

    let lowest_location = seeds
        .iter()
        .map(|seed| seed_to_location_map.map(*seed))
        .min()
        .unwrap();
    println!("{lowest_location}");

    // part 2
    let seeds: Vec<Range<u64>> = seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect();

    let location_to_seed_map = seed_to_location_map.clone().invert();

    let locations = 0..;
    let (lowest_location, seed) = locations
        .map(|l| (l, location_to_seed_map.map(l)))
        .find(|(_, seed)| seeds.iter().any(|r| r.contains(seed)))
        .unwrap();
    println!("{lowest_location} maps to {seed:?}");
}

#[derive(Debug, Clone, Copy)]
struct MapRange {
    source: u64,
    destinaton: u64,
    len: u64,
}

impl MapRange {
    fn map(&self, num: u64) -> Option<u64> {
        if !self.contains(num) {
            return None;
        }

        Some(self.destinaton + (num - self.source))
    }

    fn contains(&self, num: u64) -> bool {
        (self.source..self.source + self.len).contains(&num)
    }

    fn invert(&self) -> Self {
        Self {
            source: self.destinaton,
            destinaton: self.source,
            len: self.len,
        }
    }
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace().map(|n| n.parse().unwrap());

        Ok(Self {
            destinaton: nums.next().unwrap(),
            source: nums.next().unwrap(),
            len: nums.next().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
struct Layer {
    ranges: Vec<MapRange>,
}

impl Layer {
    fn map(&self, num: u64) -> u64 {
        let range = self.ranges.iter().find(|range| range.contains(num));

        range.map(|r| r.map(num).unwrap()).unwrap_or(num)
    }

    fn invert(&self) -> Self {
        let mut ranges = self.ranges.clone();
        ranges.iter_mut().for_each(|r| *r = r.invert());

        Self { ranges }
    }
}

impl FromStr for Layer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ignore map header
        Ok(Self {
            ranges: s.lines().skip(1).map(|m| m.parse().unwrap()).collect(),
        })
    }
}

#[derive(Debug, Clone)]
struct LayeredMap {
    layers: Vec<Layer>,
}

impl LayeredMap {
    fn map(&self, num: u64) -> u64 {
        self.layers.iter().fold(num, |acc, layer| layer.map(acc))
    }

    fn invert(&self) -> Self {
        let mut layers: Vec<Layer> = self.layers.iter().map(|l| l.invert()).collect();
        layers.reverse();

        Self { layers }
    }
}

impl FromStr for LayeredMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            layers: s.split("\n\n").map(|m| m.parse().unwrap()).collect(),
        })
    }
}
