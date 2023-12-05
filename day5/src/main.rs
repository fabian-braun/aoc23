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

    let min_loc = seeds.map(|seed| {
        let tmp = mapping[&("seed".to_string(), "soil".to_string())].lookup_src(seed);
        let tmp = mapping[&("soil".to_string(), "fertilizer".to_string())].lookup_src(tmp);
        let tmp = mapping[&("fertilizer".to_string(), "water".to_string())].lookup_src(tmp);
        let tmp = mapping[&("water".to_string(), "light".to_string())].lookup_src(tmp);
        let tmp = mapping[&("light".to_string(), "temperature".to_string())].lookup_src(tmp);
        let tmp = mapping[&("temperature".to_string(), "humidity".to_string())].lookup_src(tmp);
        let location = mapping[&("humidity".to_string(), "location".to_string())].lookup_src(tmp);
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
    pub fn lookup_src(&self, src: i64) -> i64 {
        for mapping in self.mappings.iter() {
            if src >= mapping.start_incl && src < mapping.end_excl {
                return src + mapping.offset;
            }
        }
        src
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
        assert_eq!(50, actual.lookup_src(98));
        assert_eq!(51, actual.lookup_src(99));
        assert_eq!(55, actual.lookup_src(53));
        assert_eq!(10, actual.lookup_src(10));
    }
}