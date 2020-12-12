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
    // Pairwise iteration over the endpoints = iteration over the connections
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

/// Break up a device chain into chunks where mutation is possible.
///
/// - If the difference between a pair is 3, e.g. `(3, 6)` in `[1, 2, 3, 6, 7, 8]`, then neither of
///   those elements can be removed and the permutation calculation can be performed on chunks
///   either side of that connection, e.g. `[1, 2, 3]` and `[6, 7, 8]`.
/// - If a chunk is smaller than 3 elements, there is no scope for removal, because the ends of
///   a chunk are immutable by the previous definition.
fn find_mutable_chunks(data: &[u8]) -> Vec<&[u8]> {
    let mut output = Vec::new();
    let mut chunk_start: usize = 0;
    for (i, pair) in data.windows(2).enumerate() {
        if pair[1] - pair[0] == 3 {
            let chunk_boundary = i + 1;
            let chunk = &data[chunk_start..chunk_boundary];
            if chunk.len() > 2 {
                output.push(chunk);
            }
            chunk_start = chunk_boundary;
        }
    }
    output
}

fn is_chunk_valid<I: Iterator<Item = u8>>(mut iter: I) -> bool {
    let (mut maybe_a, mut maybe_b) = (iter.next(), iter.next());
    if maybe_a.is_none() || maybe_b.is_none() {
        return false;
    }
    while let (Some(a), Some(b)) = (maybe_a, maybe_b) {
        if b - a > 3 {
            return false;
        }
        maybe_a = maybe_b;
        maybe_b = iter.next();
    }
    return true;
}

struct BinaryCounter {
    bits: Vec<bool>,
    done: bool,
}

impl BinaryCounter {
    fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut bits = Vec::with_capacity(size);
        bits.resize(size, false);
        BinaryCounter { bits, done: false }
    }
}

impl Iterator for BinaryCounter {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        let output = if self.done {
            None
        } else {
            Some(self.bits.clone())
        };
        if !self.done {
            let mut overflowed = true;
            for i in (0..self.bits.len()).rev() {
                self.bits[i] = !self.bits[i];
                if self.bits[i] {
                    overflowed = false;
                    break;
                }
            }
            self.done = overflowed;
        }
        output
    }
}

/// Given `data` where no gap between values is larger than 2, find how many permutations are valid.
///
/// The only permutation allowed is removal of chargers, because of the requirement that chargers
/// plug into other power sources that are 1-3 lower - a swap would violate that. Any combination of
/// chargers can be removed as long as they are not the first or last (those are already on an
/// immutable boundary) and as long as all pair differences remain in the range `1..=3`.
fn count_chunk_permutations(data: &[u8]) -> usize {
    // Iterate over possible keep/skip combinations for all chargers except the first and last
    BinaryCounter::new(data.len() - 2)
        .map(|mask| {
            // Build the full mask, where first and last are mandatory (already part of a 3-gap)
            std::iter::once(true)
                .chain(mask.iter().cloned())
                .chain(std::iter::once(true))
                // Combine with data
                .zip(data.iter())
                // Turn into an iterator that only yields allowed items
                .filter_map(|(allow, item)| if allow { Some(item) } else { None })
                // ... and collect that into a sequence we can test
                .cloned()
                .collect::<Vec<_>>()
        })
        // Only allow valid sequences
        .filter(|chunk| is_chunk_valid(chunk.iter().cloned()))
        // How many sequences were valid?
        .count()
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    // Power outlet
    let mut data = vec![0_u8];
    // Chargers
    data.append(&mut read_input(&input_path)?);
    // Sorted, as in part1
    data.sort();
    // Device
    data.push(data.last().unwrap() + 3);

    // Break the chain into smaller problems separated by gaps of 3
    let chunks = find_mutable_chunks(&data);
    // Total permutations = product of permutations of each "independent" chunk
    Ok(chunks
        .iter()
        .cloned()
        .map(count_chunk_permutations)
        .product::<usize>()
        .to_string())
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
    fn test_count_chunk_permutations() {
        assert_eq!(count_chunk_permutations(&[1, 2, 3]), 2);
        assert_eq!(count_chunk_permutations(&[1, 2, 3, 4]), 4);
        assert_eq!(count_chunk_permutations(&[1, 2, 3, 4, 5]), 7);
        assert_eq!(count_chunk_permutations(&[1, 2, 3, 4, 5, 6]), 13);
        assert_eq!(count_chunk_permutations(&[1, 2, 3, 4, 5, 6, 7]), 24);
    }

    #[test]
    fn test_binary_counter() {
        let mut counter = BinaryCounter::new(3);
        assert_eq!(counter.next(), Some(vec![false, false, false]));
        assert_eq!(counter.next(), Some(vec![false, false, true]));
        assert_eq!(counter.next(), Some(vec![false, true, false]));
        assert_eq!(counter.next(), Some(vec![false, true, true]));
        assert_eq!(counter.next(), Some(vec![true, false, false]));
        assert_eq!(counter.next(), Some(vec![true, false, true]));
        assert_eq!(counter.next(), Some(vec![true, true, false]));
        assert_eq!(counter.next(), Some(vec![true, true, true]));
        assert_eq!(counter.next(), None);
        assert_eq!(counter.next(), None);
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(part2(data_path!("day10_example1.txt")).unwrap(), "8");
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(part2(data_path!("day10_example2.txt")).unwrap(), "19208");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(data_path!("day10_input.txt")).unwrap(),
            "12089663946752"
        );
    }
}
