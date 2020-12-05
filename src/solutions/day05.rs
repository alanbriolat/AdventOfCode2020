use super::prelude::*;

use crate::util;

fn decode_binary(v: &[u8], one: u8) -> u16 {
    v.iter()
        .fold(0, |acc, &c| (acc << 1) | if c == one { 1 } else { 0 })
}

fn decode_seat(id: &[u8]) -> (u16, u16) {
    (decode_binary(&id[..7], b'B'), decode_binary(&id[7..], b'R'))
}

fn decode_seat_id(id: &[u8]) -> u16 {
    let (row, col) = decode_seat(id);
    (row << 3) | col
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    util::read_lines(&input_path)
        .map(|id| decode_seat_id(id.as_bytes()))
        .max()
        .ok_or("no result found".into())
        .map(|id| id.to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    let mut seats: Vec<_> = util::read_lines(&input_path)
        .map(|id| decode_seat_id(id.as_bytes()))
        .collect();
    seats.sort();
    seats
        .windows(2)
        .find_map(|pair| {
            if pair[1] == pair[0] + 2 {
                Some(pair[0] + 1)
            } else {
                None
            }
        })
        .ok_or("no result found".into())
        .map(|id| id.to_string())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day05part1", || part1(data_path!("day05_input.txt")));
    runner.add("day05part2", || part2(data_path!("day05_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_binary() {
        assert_eq!(decode_binary(b"FBFBBFF", b'B'), 44);
        assert_eq!(decode_binary(b"RLR", b'R'), 5);
    }

    #[test]
    fn test_decode_seat() {
        assert_eq!(decode_seat(b"FBFBBFFRLR"), (44, 5));
        assert_eq!(decode_seat(b"BFFFBBFRRR"), (70, 7));
        assert_eq!(decode_seat(b"FFFBBBFRRR"), (14, 7));
        assert_eq!(decode_seat(b"BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn test_decode_seat_id() {
        assert_eq!(decode_seat_id(b"FBFBBFFRLR"), 357);
        assert_eq!(decode_seat_id(b"BFFFBBFRRR"), 567);
        assert_eq!(decode_seat_id(b"FFFBBBFRRR"), 119);
        assert_eq!(decode_seat_id(b"BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day05_input.txt")).unwrap(), "915");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day05_input.txt")).unwrap(), "699");
    }
}
