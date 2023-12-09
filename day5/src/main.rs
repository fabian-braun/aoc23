use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::str::FromStr;

use itertools::Itertools;
use sorted_vec::SortedVec;

fn main() {
    let input = read_to_string("input_day5").unwrap();
    let lines = input.split("\n\n").collect_vec();
    let seeds = lines[0].split_whitespace().dropping(1).map(i64::from_str).filter_map(Result::ok).collect_vec();
    let seeds = seeds.chunks_exact(2)
        .map(|chunk| {
            let start = chunk[0];
            let len = chunk[1];
            (start, start + len)
        }).collect_vec();
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

    let min_loc = seeds.into_iter().map(|(range_start_incl, range_end_excl)| {
        let tmp = seed_to_soil.to_dst_ranges(range_start_incl, range_end_excl);
        let tmp = tmp.into_iter().flat_map(|(range_start_incl, range_end_excl)| soil_to_fertilizer.to_dst_ranges(range_start_incl, range_end_excl)).collect_vec();
        let tmp = tmp.into_iter().flat_map(|(range_start_incl, range_end_excl)| fertilizer_to_water.to_dst_ranges(range_start_incl, range_end_excl)).collect_vec();
        let tmp = tmp.into_iter().flat_map(|(range_start_incl, range_end_excl)| water_to_light.to_dst_ranges(range_start_incl, range_end_excl)).collect_vec();
        let tmp = tmp.into_iter().flat_map(|(range_start_incl, range_end_excl)| light_to_temperature.to_dst_ranges(range_start_incl, range_end_excl)).collect_vec();
        let tmp = tmp.into_iter().flat_map(|(range_start_incl, range_end_excl)| temperature_to_humidity.to_dst_ranges(range_start_incl, range_end_excl)).collect_vec();
        let tmp = tmp.into_iter().flat_map(|(range_start_incl, range_end_excl)| humidity_to_location.to_dst_ranges(range_start_incl, range_end_excl)).collect_vec();
        let min = tmp.into_iter().map(|(range_start_incl, range_end_excl)| {
            range_start_incl.min(range_end_excl - 1)
        }).min();
        println!("{:?}", min);
        min
    }).min().unwrap();

    println!("Part I: The result is {min_loc:?}");
}

#[derive(PartialEq, Eq, Debug)]
struct Mapping {
    src: String,
    dst: String,
    mappings: SortedVec<IndexMapping>,
}

impl Display for Mapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} -> {}\n", self.src, self.dst))?;
        self.mappings.iter().try_for_each(|m| {
            f.write_str(&format!("{:03}..{:03} ", m.start_incl, m.end_excl))
        })?;
        f.write_str("\n")?;
        self.mappings.iter().try_for_each(|m| {
            f.write_str(&format!("{:03}..{:03} ", m.start_incl + m.offset, m.end_excl + m.offset))
        })?;
        Ok(())
    }
}

impl Mapping {
    pub fn to_dst(&self, src: i64) -> i64 {
        match self.find_mapping(src) {
            Ok(m) => { src + m.offset }
            Err(_) => { src }
        }
    }

    pub fn find_mapping(&self, src: i64) -> Result<IndexMapping, Option<IndexMapping>> {
        let idx = self.mappings.binary_search_by(|mapping| {
            if src >= mapping.start_incl && src < mapping.end_excl {
                Ordering::Equal
            } else if src < mapping.start_incl {
                Ordering::Greater
            } else { Ordering::Less }
        });
        match idx {
            Ok(idx) => { Ok(self.mappings[idx]) }
            Err(idx) => { Err(self.mappings.get(idx).copied()) }
        }
    }

    pub fn to_dst_ranges(&self, mut src_start_incl: i64, src_end_excl: i64) -> Vec<(i64, i64)> {
        let mut ranges: Vec<(i64, i64)> = vec![];
        while src_start_incl < src_end_excl {
            match self.find_mapping(src_start_incl) {
                Ok(m) => {
                    let dst_start_incl = src_start_incl + m.offset;
                    let dst_end_excl = if src_end_excl <= m.end_excl {
                        src_start_incl = src_end_excl; // stop loop
                        src_end_excl + m.offset
                    } else {
                        src_start_incl = m.end_excl; // continue mapping
                        m.end_excl + m.offset
                    };
                    ranges.push((dst_start_incl, dst_end_excl));
                }
                Err(nm) => {
                    match nm {
                        Some(nm) => {
                            ranges.push((src_start_incl, nm.start_incl));
                            src_start_incl = nm.start_incl;
                        }
                        None => {
                            ranges.push((src_start_incl, src_end_excl));
                            src_start_incl = src_end_excl; // stop loop
                        }
                    }
                }
            };
        };
        ranges
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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
        assert_eq!(vec![(10, 50), (52, 100), (50, 52)], actual.to_dst_ranges(10, 100));
    }
}