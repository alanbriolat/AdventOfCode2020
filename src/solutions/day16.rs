use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::{Deref, RangeInclusive};
use std::str::FromStr;

use super::prelude::*;
use crate::util;

fn read_range_inclusive(s: &str) -> crate::Result<RangeInclusive<u16>> {
    let (raw_start, raw_end) = util::str_partition(s, "-");
    Ok(RangeInclusive::new(raw_start.parse()?, raw_end.parse()?))
}

struct Rule {
    field: String,
    range_a: RangeInclusive<u16>,
    range_b: RangeInclusive<u16>,
}

impl Rule {
    fn contains(&self, v: u16) -> bool {
        self.range_a.contains(&v) || self.range_b.contains(&v)
    }
}

impl FromStr for Rule {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (field, raw_ranges) = util::str_partition(s, ": ");
        let (raw_range_a, raw_range_b) = util::str_partition(raw_ranges, " or ");
        let range_a = read_range_inclusive(raw_range_a)?;
        let range_b = read_range_inclusive(raw_range_b)?;
        Ok(Rule {
            field: field.to_owned(),
            range_a,
            range_b,
        })
    }
}

struct Ruleset(Vec<Rule>);

impl Ruleset {
    fn matching_fields_for_value(&self, v: u16) -> impl Iterator<Item = &str> + '_ {
        self.0.iter().filter_map(move |rule| {
            if rule.contains(v) {
                Some(rule.field.as_str())
            } else {
                None
            }
        })
    }

    fn error_rate(&self, ticket: &Ticket) -> u64 {
        ticket
            .iter()
            .filter_map(|&v| {
                if let None = self.matching_fields_for_value(v).next() {
                    Some(v as u64)
                } else {
                    None
                }
            })
            .sum()
    }

    fn is_valid(&self, ticket: &Ticket) -> bool {
        self.error_rate(ticket) == 0
    }
}

struct Ticket(Vec<u16>);

impl FromStr for Ticket {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split(",")
            .map(|x| x.parse::<u16>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Ticket(numbers))
    }
}

impl Deref for Ticket {
    type Target = Vec<u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Input {
    ruleset: Ruleset,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn read_input(input_path: &PathBuf) -> crate::Result<Input> {
    let lines: Vec<_> = util::read_lines(input_path).collect();
    let breaks: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, s)| if s == "" { Some(i) } else { None })
        .collect();
    let rules = lines[0..breaks[0]]
        .iter()
        .map(|line| line.parse::<Rule>())
        .collect::<Result<Vec<_>, _>>()?;
    let ruleset = Ruleset(rules);
    let ticket = lines[breaks[0] + 2].parse::<Ticket>()?;
    let nearby_tickets = lines[(breaks[1] + 2)..]
        .iter()
        .map(|line| line.parse::<Ticket>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input {
        ruleset,
        ticket,
        nearby_tickets,
    })
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let input = read_input(&input_path)?;
    let error_rate = input
        .nearby_tickets
        .iter()
        // Map each ticket its "scanning error rate"
        .map(|ticket| input.ruleset.error_rate(ticket))
        .sum::<u64>();
    Ok(error_rate.to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    let input = read_input(&input_path)?;

    // TODO: this is kinda duplicating some checks, but is it better or worse than extra
    //  allocations for data that might be thrown away?
    let valid_tickets: Vec<_> = input
        .nearby_tickets
        .iter()
        .filter(|t| input.ruleset.is_valid(t))
        .collect();

    // For each field position, mapping of field name to how many tickets have a valid value for that field name
    let mut match_counts: Vec<HashMap<&str, usize>> = Vec::with_capacity(input.ticket.len());
    match_counts.resize_with(input.ticket.len(), HashMap::new);

    // Find out how many tickets have valid values for each position/field combination
    for ticket in valid_tickets.iter() {
        for (i, &v) in ticket.iter().enumerate() {
            for field in input.ruleset.matching_fields_for_value(v) {
                *match_counts[i].entry(field).or_default().borrow_mut() += 1;
            }
        }
    }

    // Find the correct field mapping: a field corresponds to a position if all values in that
    // position match the rules for that field
    let mut field_mapping: HashMap<&str, usize> = HashMap::new();
    while field_mapping.len() < input.ruleset.0.len() {
        let mut changed = 0_usize;
        for (i, counts) in match_counts.iter().enumerate() {
            let candidates: Vec<_> = counts
                .iter()
                .filter_map(|(&field, &count)| {
                    if count == valid_tickets.len() && !field_mapping.contains_key(field) {
                        Some(field)
                    } else {
                        None
                    }
                })
                .collect();
            if candidates.len() == 1 {
                field_mapping.insert(candidates[0], i);
                changed += 1;
            }
        }
        // Give up if we can't map any more
        if changed == 0 {
            break;
        }
    }

    // Get just the fields we're interested in (and check we found them all)
    let departure_fields: Vec<_> = field_mapping
        .keys()
        .cloned()
        .filter(|k| k.starts_with("departure"))
        .collect();
    assert_eq!(departure_fields.len(), 6);

    // Get the product of these interesting fields
    Ok(departure_fields
        .iter()
        .map(|k| field_mapping[k])
        .map(|i| input.ticket[i] as u64)
        .product::<u64>()
        .to_string())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day16part1", || part1(data_path!("day16_input.txt")));
    runner.add("day16part2", || part2(data_path!("day16_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(data_path!("day16_example1.txt")).unwrap(), "71");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day16_input.txt")).unwrap(), "27870");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(data_path!("day16_input.txt")).unwrap(),
            "3173135507987"
        );
    }
}
