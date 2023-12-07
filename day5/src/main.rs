use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

use itertools::Itertools;
use sorted_vec::SortedVec;

fn main() {
    let input = read_to_string("example_day5").unwrap();
    let lines = input.split("\n\n").collect_vec();
    let seeds = lines[0].split_whitespace().dropping(1).map(i64::from_str).filter_map(Result::ok).collect_vec();
    let seeds = seeds.chunks_exact(2)
        .map(|chunk| {
            let start = chunk[0];
            let len = chunk[1];
            start..start + len
        }).flatten();
    let mapping: HashMap<(String, String), Mapping> = lines.iter().dropping(1).map(|l| {
        str_to_mapping(l)
    }).map(|m| {
        ((m.src.clone(), m.dst.clone()), m)
    }).collect();

    println!("mappings initialized");

    let seed_to_soil = &mapping[&("seed".to_string(), "soil".to_string())];
    let soil_to_fertilizer = &mapping[&("soil".to_string(), "fertilizer".to_string())];
    let fertilizer_to_water = &mapping[&("fertilizer".to_string(), "water".to_string())];
    let water_to_light = &mapping[&("water".to_string(), "light".to_string())];
    let light_to_temperature = &mapping[&("light".to_string(), "temperature".to_string())];
    let temperature_to_humidity = &mapping[&("temperature".to_string(), "humidity".to_string())];
    let humidity_to_location = &mapping[&("humidity".to_string(), "location".to_string())];

    let min_loc = seeds.map(|seed| {
        let tmp =      seed_to_soil.to_dst(seed);
        let tmp =      soil_to_fertilizer.to_dst(tmp);
        let tmp =      fertilizer_to_water.to_dst(tmp);
        let tmp =      water_to_light.to_dst(tmp);
        let tmp =      light_to_temperature.to_dst(tmp);
        let tmp =      temperature_to_humidity.to_dst(tmp);
        let location = humidity_to_location.to_dst(tmp);
        location
    }).min().unwrap();

    println!("Part I: The result is {min_loc}");
}

#[derive(PartialEq, Eq, Debug)]
struct Mapping {
    src: String,
    dst: String,
    mappings: SortedVec<IndexMapping>,
}

impl Mapping {
    pub fn to_dst(&self, src: i64) -> i64 {
        let idx = self.mappings.binary_search_by(|mapping| {
            if src >= mapping.start_incl && src < mapping.end_excl {
                Ordering::Equal
            } else if src < mapping.start_incl {
                Ordering::Greater
            } else { Ordering::Less }
        });
        match idx {
            Ok(idx) => { src + self.mappings[idx].offset }
            Err(_) => { src }
        }
    }
    pub fn to_src(&self, dst: i64) -> i64 {
        let idx = self.mappings.binary_search_by(|mapping| {
            if dst >= mapping.start_incl + mapping.offset && dst < mapping.end_excl + mapping.offset {
                Ordering::Equal
            } else if dst < mapping.start_incl + mapping.offset {
                Ordering::Greater
            } else { Ordering::Less }
        });
        match idx {
            Ok(idx) => { dst - self.mappings[idx].offset }
            Err(_) => { dst }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct IndexMapping {
    pub start_incl: i64,
    pub end_excl: i64,
    pub offset: i64,
}

fn str_to_mapping(s: &str) -> Mapping {
    // example:
    // seed-to-soil map:\n50 98 2\n52 50 48
    let (map_type, numbers) = s.split_once(" map:\n").unwrap();
    let (src, dst) = map_type.split_once("-to-").unwrap();
    let mappings: SortedVec<IndexMapping> = numbers.split('\n').map(|r| {
        let (dst, src, len) = r.split_whitespace().map(i64::from_str).filter_map(Result::ok).collect_tuple().unwrap();
        let offset = dst - src;
        let start_incl = src;
        let end_excl = src + len;
        IndexMapping {
            start_incl,
            end_excl,
            offset,
        }
    }).collect_vec().into();
    Mapping {
        src: src.to_string(),
        dst: dst.to_string(),
        mappings,
    }
}

#[cfg(test)]
mod tests {
    use crate::{IndexMapping, Mapping, str_to_mapping};

    #[test]
    fn test_something() {
        let input = "seed-to-soil map:\n50 98 2\n52 50 48";
        let expected = Mapping {
            src: "seed".to_string(),
            dst: "soil".to_string(),
            mappings: vec![IndexMapping {
                start_incl: 50,
                end_excl: 98,
                offset: 2,
            }, IndexMapping {
                start_incl: 98,
                end_excl: 100,
                offset: -48,
            }].into(),
        };
        let actual = str_to_mapping(input);
        assert_eq!(expected, actual);
        assert_eq!(50, actual.to_dst(98));
        assert_eq!(51, actual.to_dst(99));
        assert_eq!(55, actual.to_dst(53));
        assert_eq!(10, actual.to_dst(10));
    }
}