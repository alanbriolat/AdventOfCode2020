use super::prelude::*;
use crate::util;

struct Cypher {
    /// How many previous numbers are considered during validating a new number.
    window_size: usize,
    /// The next cell/chunk to overwrite in the circular buffers.
    next_index: usize,
    /// Circular buffer of the most recent numbers.
    recent: Vec<u64>,
    /// Circular buffer of all valid next numbers.
    ///
    /// Size is `window_size.pow(2)`. This is actually ~2x the size it needs to be: addition is
    /// commutative, so the number of valid next numbers is the nth triangular number (no need to
    /// have both `a + b` and `b + a`), however having a nicely divisible array makes rolling
    /// updates to a circular buffer much easier to calculate.
    ///
    /// Uses `Option<u64>` instead of just `u64` so that the "diagonal" can be marked as invalid.
    valid: Vec<Option<u64>>,
}

impl Cypher {
    fn new(preamble: &[u64]) -> Self {
        assert!(preamble.len() >= 2);

        let window_size = preamble.len();
        let mut recent = Vec::with_capacity(window_size);
        recent.resize(window_size, 0);
        // The "diagonal" is invalid, because the number summed have to be different numbers
        let valid = (0..window_size)
            .flat_map(|i| (0..window_size).map(move |j| if i == j { None } else { Some(0) }))
            .collect();
        let mut cypher = Cypher {
            window_size,
            next_index: 0,
            recent,
            valid,
        };
        for next in preamble.iter().cloned() {
            cypher.update(next);
        }
        cypher
    }

    fn update(&mut self, next: u64) {
        let i = self.next_index;

        // Swap in the new number at the next position in the circular buffer of recent numbers
        let old = self.recent[self.next_index];
        self.recent[self.next_index] = next;

        // Update the contiguous chunk of valid numbers that should include this number
        for cell in &mut self.valid[(i * self.window_size)..((i + 1) * self.window_size)] {
            if let Some(x) = cell {
                *x -= old;
                *x += next;
            }
        }

        // Update the numbers that are in the other chunks
        for chunk in self.valid.chunks_mut(self.window_size) {
            if let Some(x) = &mut chunk[i] {
                *x -= old;
                *x += next;
            }
        }

        // Update next index to use in the circular buffers
        self.next_index = (i + 1) % self.window_size;
    }

    fn is_valid(&self, next: u64) -> bool {
        self.valid.contains(&Some(next))
    }

    fn accept(&mut self, next: u64) -> bool {
        let valid = self.is_valid(next);
        self.update(next);
        valid
    }
}

fn read_input(input_path: &PathBuf) -> crate::Result<Vec<u64>> {
    util::read_lines(input_path)
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(crate::Error::from)
}

fn part1_impl(data: Vec<u64>, window_size: usize) -> crate::Result<u64> {
    let mut cypher = Cypher::new(&data[..window_size]);
    for next in data[window_size..].iter().cloned() {
        if !cypher.accept(next) {
            return Ok(next);
        }
    }
    Err("no invalid numbers found".into())
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let data = read_input(&input_path)?;
    part1_impl(data, 25).map(|result| result.to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day09part1", || part1(data_path!("day09_input.txt")));
    runner.add("day09part2", || part2(data_path!("day09_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_impl_example() {
        let data = read_input(&data_path!("day09_example1.txt")).unwrap();
        assert_eq!(part1_impl(data, 5).unwrap(), 127);
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day09_input.txt")).unwrap(), "133015568");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day09_input.txt")).unwrap(), "");
    }
}
