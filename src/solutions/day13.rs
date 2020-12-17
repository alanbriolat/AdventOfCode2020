use num::Integer;

use super::prelude::*;
use crate::util;

struct Data {
    departure: i64,
    buses: Vec<Option<i64>>,
}

fn read_input(input_path: &PathBuf) -> crate::Result<Data> {
    let mut lines = util::read_lines(input_path);
    let departure = lines.next().expect("departure time").parse()?;
    let buses = lines
        .next()
        .expect("bus list")
        .split(",")
        .map(|b| {
            if b == "x" {
                None
            } else {
                Some(b.parse::<i64>().unwrap())
            }
        })
        .collect::<Vec<_>>();
    Ok(Data { departure, buses })
}

/// Part 1: "What is the ID of the earliest bus you can take to the airport multiplied by the number
/// of minutes you'll need to wait for that bus?"
///
/// Use remainder of division of the departure time by the bus frequency to find the delay for each
/// bus, sort by the delay, take the first one.
fn part1(input_path: PathBuf) -> crate::Result<String> {
    let data = read_input(&input_path)?;
    let mut departures: Vec<_> = data
        .buses
        .iter()
        .filter_map(|bus| {
            bus.map(|bus| {
                let (_, rem) = data.departure.div_rem(&bus);
                let delay = bus - rem;
                (bus, delay)
            })
        })
        .collect();
    departures.sort_by_key(|(_bus, delay)| *delay);
    let (bus, delay) = departures[0];
    Ok((bus * delay).to_string())
}

#[allow(unused_variables)]
fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day13part1", || part1(data_path!("day13_input.txt")));
    runner.add("day13part2", || part2(data_path!("day13_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(data_path!("day13_example1.txt")).unwrap(), "295");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day13_input.txt")).unwrap(), "1895");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day13_input.txt")).unwrap(), "");
    }
}
