use super::prelude::*;
use crate::util;

fn part1(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day16part1", || part1(data_path!("day16_input.txt")));
    runner.add("day16part2", || part2(data_path!("day16_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day16_input.txt")).unwrap(), "");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day16_input.txt")).unwrap(), "");
    }
}