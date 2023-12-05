fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let (seeds, maps) = parse_input(input);
    seeds.into_iter().map(|seed| find_location(seed, &maps)).min().unwrap()
}

fn find_location(seed: usize, maps: &Vec<Map>) -> usize {
    let mut current = seed;
    for map in maps {
        current = map.find_dest(current);
    }
    current
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Map>) {
    let (seeds, rest) = input.split_once("\n\n").unwrap();
    (parse_seeds(seeds), parse_maps(rest))
}

fn parse_seeds(seeds: &str) -> Vec<usize> {
    seeds
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse_maps(maps: &str) -> Vec<Map> {
    maps.split("\n\n").map(parse_map).collect()
}

fn parse_map(map: &str) -> Map {
    Map {
        ranges: map.lines().skip(1).map(parse_range).collect(),
    }
}

fn parse_range(range: &str) -> Range {
    let mut nums = range.split(' ').map(str::parse).map(Result::unwrap);
    Range {
        destination_start: nums.next().unwrap(),
        source_start: nums.next().unwrap(),
        length: nums.next().unwrap(),
    }
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn find_dest(&self, src: usize) -> usize {
        for range in &self.ranges {
            if let Some(dest) = range.find_dest(src) {
                return dest;
            }
        }
        src
    }
}

struct Range {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl Range {
    fn find_dest(&self, src: usize) -> Option<usize> {
        if src >= self.source_start && src < self.source_start + self.length {
            Some(self.destination_start + src - self.source_start)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() {
        const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(run_1(INPUT), 35);
    }
}
