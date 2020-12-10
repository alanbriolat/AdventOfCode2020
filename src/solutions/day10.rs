use super::prelude::*;
use crate::util;

fn read_input(input_path: &PathBuf) -> crate::Result<Vec<u8>> {
    util::read_lines(input_path)
        .map(|line| line.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(crate::Error::from)
}

/// From a slice of "joltage" values, find an order to connect them in, and return how many 1-jolt
/// and 3-jolt steps there are in the chain as a tuple.
///
/// Given the constraints that `x[i+1] - x` must be in the range `1..=3`, the ordering is found by a
/// simple sort. Then the connections can be processed with a pairwise iteration over the sorted
/// data.
fn part1_impl(data: &[u8]) -> crate::Result<(usize, usize)> {
    let mut data = Vec::from(data);
    data.sort();
    let mut counts = vec![0_usize; 4];
    let outlet = 0_u8;
    let device = data.last().unwrap() + 3;
    let prev_iter = std::iter::once(outlet).chain(data[..].iter().cloned());
    let next_iter = data[..].iter().cloned().chain(std::iter::once(device));
    for (a, b) in prev_iter.zip(next_iter) {
        let diff = b - a;
        if diff <= 3 {
            counts[diff as usize] += 1;
        } else {
            return Err(format!("gap too large: {} -> {}", a, b).into());
        }
    }
    Ok((counts[1], counts[3]))
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let data = read_input(&input_path)?;
    let result = part1_impl(&data)?;
    Ok((result.0 * result.1).to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day10part1", || part1(data_path!("day10_input.txt")));
    runner.add("day10part2", || part2(data_path!("day10_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_impl_example1() {
        let data = read_input(&data_path!("day10_example1.txt")).unwrap();
        assert_eq!(part1_impl(&data).unwrap(), (7, 5));
    }

    #[test]
    fn test_part1_impl_example2() {
        let data = read_input(&data_path!("day10_example2.txt")).unwrap();
        assert_eq!(part1_impl(&data).unwrap(), (22, 10));
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day10_input.txt")).unwrap(), "2312");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day10_input.txt")).unwrap(), "");
    }
}