#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::Itertools;
use std::ops::Range;

const INPUT: &[u8] = include_bytes!("input.txt");

struct Input {
    seeds: Vec<u64>,
    seed_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>,
}

fn parse_section(input: &[u8]) -> Vec<(u64, u64, u64)> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            line.split_str(" ")
                .map(|n| unsafe { n.to_str_unchecked() }.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

impl Input {
    fn parse(input: &[u8]) -> Self {
        let mut sections = input.split_str("\n\n");
        let seeds = sections
            .next()
            .unwrap()
            .strip_prefix(b"seeds: ")
            .unwrap()
            .split_str(" ")
            .map(|n| unsafe { n.to_str_unchecked() }.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let seed_to_soil = parse_section(sections.next().unwrap());
        let soil_to_fertilizer = parse_section(sections.next().unwrap());
        let fertilizer_to_water = parse_section(sections.next().unwrap());
        let water_to_light = parse_section(sections.next().unwrap());
        let light_to_temperature = parse_section(sections.next().unwrap());
        let temperature_to_humidity = parse_section(sections.next().unwrap());
        let humidity_to_location = parse_section(sections.next().unwrap());

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

fn map_to_next(map: &[(u64, u64, u64)], src: u64) -> u64 {
    map.iter()
        .find_map(|&(dst_range, src_range, range_len)| {
            (src_range..(src_range + range_len))
                .contains(&src)
                .then_some(dst_range + (src - src_range))
        })
        .unwrap_or(src)
}

fn part_1(parsed: &Input) -> u64 {
    parsed
        .seeds
        .iter()
        .map(|&seed| {
            let soil = map_to_next(&parsed.seed_to_soil, seed);
            let fertilizer = map_to_next(&parsed.soil_to_fertilizer, soil);
            let water = map_to_next(&parsed.fertilizer_to_water, fertilizer);
            let light = map_to_next(&parsed.water_to_light, water);
            let temperature = map_to_next(&parsed.light_to_temperature, light);
            let humidity = map_to_next(&parsed.temperature_to_humidity, temperature);

            map_to_next(&parsed.humidity_to_location, humidity)
        })
        .min()
        .unwrap()
}

fn multi_map_to_next(map: &[(u64, u64, u64)], src_range: Range<u64>) -> Vec<Range<u64>> {
    let src_start = src_range.start;
    let src_end = src_range.end - 1;
    for &(dst_range, src_range, range_len) in map {
        let rule_range = src_range..(src_range + range_len);
        if !rule_range.contains(&src_start) && !rule_range.contains(&src_end) {
            continue;
        }
        if rule_range.contains(&src_start) && rule_range.contains(&src_end) {
            let start = dst_range + (src_start - src_range);
            let end = dst_range + (src_end - src_range);
            #[allow(clippy::single_range_in_vec_init)]
            return vec![start..end];
        }

        let mut result = Vec::new();
        if rule_range.contains(&src_start) {
            let start = dst_range + (src_start - src_range);
            let end = dst_range + range_len;
            result.push(start..end);
            let remainder_range = (src_range + range_len)..src_end;
            result.extend(multi_map_to_next(map, remainder_range));
        } else {
            let start = dst_range;
            let end = dst_range + (src_end - src_range);
            result.push(start..end);
            let remainder_range = src_start..src_range;
            result.extend(multi_map_to_next(map, remainder_range));
        }
        return result;
    }
    vec![src_range]
}

fn consilidate_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.sort_by_key(|r| r.start);
    let mut result = Vec::new();
    let mut current = ranges.pop().unwrap();
    while let Some(next) = ranges.pop() {
        if current.end == next.start {
            current = current.start..next.end;
        } else {
            result.push(current);
            current = next;
        }
    }
    result.push(current);
    result
}

fn part_2(parsed: &Input) -> u64 {
    let seeds = consilidate_ranges(
        parsed
            .seeds
            .chunks_exact(2)
            .map(|a| a[0]..(a[0] + a[1]))
            .collect::<Vec<_>>(),
    );

    let soil = consilidate_ranges(seeds)
        .into_iter()
        .flat_map(|seed| multi_map_to_next(&parsed.seed_to_soil, seed))
        .collect::<Vec<_>>();

    let fertilizer = consilidate_ranges(soil)
        .into_iter()
        .flat_map(|seed| multi_map_to_next(&parsed.soil_to_fertilizer, seed))
        .collect::<Vec<_>>();

    let water = consilidate_ranges(fertilizer)
        .into_iter()
        .flat_map(|seed| multi_map_to_next(&parsed.fertilizer_to_water, seed))
        .collect::<Vec<_>>();

    let light = consilidate_ranges(water)
        .into_iter()
        .flat_map(|seed| multi_map_to_next(&parsed.water_to_light, seed))
        .collect::<Vec<_>>();

    let temperature = consilidate_ranges(light)
        .into_iter()
        .flat_map(|seed| multi_map_to_next(&parsed.light_to_temperature, seed))
        .collect::<Vec<_>>();

    let humidity = consilidate_ranges(temperature)
        .into_iter()
        .flat_map(|seed| multi_map_to_next(&parsed.temperature_to_humidity, seed))
        .collect::<Vec<_>>();

    consilidate_ranges(humidity)
        .into_iter()
        .flat_map(|seed| multi_map_to_next(&parsed.humidity_to_location, seed))
        .map(|r| r.start)
        .min()
        .unwrap()
}

fn main() {
    let parsed = Input::parse(INPUT);
    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"seeds: 79 14 55 13

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

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&Input::parse(EXAMPLE)), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&Input::parse(EXAMPLE)), 46);
    }

    #[bench]
    fn bench_parse(b: &mut test::Bencher) {
        b.iter(|| Input::parse(INPUT));
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        let parsed = Input::parse(INPUT);
        b.iter(|| part_1(black_box(&parsed)));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        let parsed = Input::parse(INPUT);
        b.iter(|| part_2(black_box(&parsed)));
    }
}
