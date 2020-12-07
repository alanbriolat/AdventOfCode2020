use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use super::prelude::*;
use crate::util;

struct Rule {
    outer: String,
    inner: HashMap<String, u8>,
}

impl FromStr for Rule {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (outer_raw, inner_raw) = util::str_partition(s, " bags contain ");
        let outer: String = outer_raw.to_owned();
        let inner = if inner_raw == "no other bags." {
            HashMap::new()
        } else {
            inner_raw
                .split(", ")
                .map(|def| -> Result<(String, u8), Self::Err> {
                    // Strip trailing "bag(s?)"
                    let (def, _) = util::str_rpartition(def, " ");
                    let (count_raw, bag_def_raw) = util::str_partition(def, " ");
                    let count: u8 = count_raw.parse()?;
                    let bag_def: String = bag_def_raw.to_owned();
                    Ok((bag_def, count))
                })
                .collect::<Result<_, _>>()?
        };
        Ok(Rule { outer, inner })
    }
}

struct Ruleset {
    rules: Vec<Rule>,
}

impl Ruleset {
    fn reverse_dependencies(&self) -> HashMap<&String, HashSet<&String>> {
        let mut deps: HashMap<&String, HashSet<&String>> = HashMap::new();
        for rule in self.rules.iter() {
            for inner in rule.inner.keys() {
                deps.entry(inner).or_default().insert(&rule.outer);
            }
        }
        deps
    }

    /// Find all possible types of bag that might contain `target` at any level of nesting.
    ///
    /// If the ruleset is treated as a dependency tree of outer bags to inner bags, then this
    /// solution is expressed as a breadth-first search of the *inverse* dependency tree,
    /// starting at `target`.
    fn find_all_outers(&self, target: &String) -> HashSet<&String> {
        // Build inverse dependency tree
        let deps = self.reverse_dependencies();
        // Find everything reachable from `child`
        let mut queue: VecDeque<&String> = VecDeque::from(vec![target]);
        let mut found: HashSet<&String> = HashSet::new();
        while let Some(next) = queue.pop_front() {
            if let Some(bags) = deps.get(next) {
                for &bag in bags {
                    if found.insert(bag) {
                        queue.push_back(bag);
                    }
                }
            }
        }
        found
    }
}

fn read_input(input_path: &PathBuf) -> crate::Result<Ruleset> {
    util::read_lines(input_path)
        .map(|line| line.parse::<Rule>())
        .collect::<Result<_, _>>()
        .map(|rules| Ruleset { rules })
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let ruleset = read_input(&input_path)?;
    let ancestors = ruleset.find_all_outers(&("shiny gold".to_owned()));
    Ok(ancestors.len().to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day07part1", || part1(data_path!("day07_input.txt")));
    runner.add("day07part2", || part2(data_path!("day07_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(data_path!("day07_example.txt")).unwrap(), "4");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day07_input.txt")).unwrap(), "179");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day07_input.txt")).unwrap(), "");
    }
}
