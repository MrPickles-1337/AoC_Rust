use rayon::prelude::*;

#[derive(Debug, Default)]
pub struct Map {
    src: Vec<u64>,
    dst: Vec<u64>,
    len: Vec<u64>,
}

impl Map {
    pub fn add(&mut self, src: u64, dst: u64, len: u64) {
        self.src.push(src);
        self.dst.push(dst);
        self.len.push(len);
    }

    pub fn map_value(&self, value: &mut u64) {
        for i in 0..self.src.len() {
            let src = self.src[i];
            let dst = self.dst[i];
            let len = self.len[i];
            if *value >= src && *value <= src + len {
                let diff = *value - src;
                *value = dst + diff;
                return;
            }
        }
    }
}

#[aoc_generator(day5)]
pub fn inptu_generator(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut split = input.split("\n\n");
    let mut maps = Vec::new();
    let seeds = split
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|a| a.parse::<u64>().unwrap())
        .collect();
    while let Some(m) = split.next() {
        let mut map = Map::default();
        m.lines().skip(1).for_each(|l| {
            let mut split = l.split(" ");
            let dst: u64 = split.next().unwrap().parse().unwrap();
            let src: u64 = split.next().unwrap().parse().unwrap();
            let len: u64 = split.next().unwrap().parse().unwrap();
            map.add(src, dst, len);
        });
        maps.push(map);
    }
    (seeds, maps)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<u64>, Vec<Map>)) -> u64 {
    input
        .0
        .clone()
        .iter_mut()
        .map(|seed| {
            for i in input.1.iter() {
                i.map_value(seed);
            }
            *seed
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<u64>, Vec<Map>)) -> u64 {
    input
        .0
        .clone()
        .into_par_iter()
        .chunks(2)
        .flat_map(|pair| {
            let start = *pair.first().unwrap();
            let len = *pair.last().unwrap();
            (start..start + len).into_par_iter()
        })
        .map(|mut seed| {
            for i in input.1.iter() {
                i.map_value(&mut seed);
            }
            seed
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "seeds: 79 14 55 13

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
        assert_eq!(35, part1(&inptu_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "seeds: 79 14 55 13

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
        assert_eq!(46, part2(&inptu_generator(input)));
    }
}
