use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let races = vec![
        Race {
            time: 42,
            record: 308,
        },
        Race {
            time: 89,
            record: 1170,
        },
        Race {
            time: 91,
            record: 1291,
        },
        Race {
            time: 89,
            record: 1467,
        },
    ];
    let big_race = Race {
        time: 42899189,
        record: 308117012911467,
    };

    println!(
        "Part I: {}",
        races
            .iter()
            .map(|m| m.record_beating_ways())
            .product::<usize>()
    );
    println!("Part II: {}", big_race.record_beating_ways());
}

struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn record_beating_ways(&self) -> usize {
        let total_time = self.time;
        let record = self.record;
        // half the time gives the optimal distance
        let mut hold_upper = total_time / 2;
        let mut hold_lower = 0usize;
        while hold_upper != hold_lower {
            let mid = (hold_upper + hold_lower) / 2;
            println!("{} {}", hold_lower, hold_upper);
            let distance_mid = (total_time - mid) * mid;
            if distance_mid <= record {
                hold_lower = mid + 1
            } else {
                hold_upper = mid
            }
        }
        let x = total_time / 2;
        let y = (total_time + 1) % 2;
        (x - hold_upper + 1) * 2 - y
    }
}

#[cfg(test)]
mod tests {
    use crate::Race;

    #[test]
    fn test_something() {
        let race1 = Race { time: 7, record: 9 };
        let race2 = Race {
            time: 15,
            record: 40,
        };
        let race3 = Race {
            time: 30,
            record: 200,
        };
        let race4 = Race {
            time: 71530,
            record: 940200,
        };
        assert_eq!(4, race1.record_beating_ways(), "race 1 failed");
        assert_eq!(8, race2.record_beating_ways(), "race 2 failed");
        assert_eq!(9, race3.record_beating_ways(), "race 3 failed");
        assert_eq!(71503, race4.record_beating_ways(), "race 4 failed");
    }
}
