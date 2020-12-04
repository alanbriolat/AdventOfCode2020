use super::prelude::*;
use crate::util::{Rect, Vector2D};

const OPEN: u8 = '.' as u8;
const TREE: u8 = '#' as u8;
const NEWLINE: u8 = '\n' as u8;

#[derive(Clone, Copy, Debug)]
enum Tile {
    Open,
    Tree,
}

struct Map {
    tiles: Vec<Tile>,
    bounds: Rect<usize>,
}

impl Map {
    fn at(&self, point: Vector2D<usize>) -> Option<Tile> {
        let index = self.bounds.row_major_index(point)?;
        Some(self.tiles[index])
    }

    fn traverse(&self, slope: Vector2D<usize>) -> impl Iterator<Item = Tile> + '_ {
        let start: Vector2D<usize> = Vector2D(0, 0);
        std::iter::successors(Some(start), move |&p| {
            let new = self.bounds.wrap_x(p + slope);
            if self.bounds.contains(new) {
                Some(new)
            } else {
                None
            }
        })
        .map(move |p| self.at(p).unwrap())
    }
}

fn read_input(input_path: PathBuf) -> crate::Result<Map> {
    let file = File::open(input_path)?;
    let mut width: Option<usize> = None;
    let data: Vec<Tile> = file
        .bytes()
        .enumerate()
        .filter_map(|(i, b)| match b.expect("read error") {
            OPEN => Some(Tile::Open),
            TREE => Some(Tile::Tree),
            NEWLINE => {
                if width.is_none() {
                    width = Some(i);
                }
                None
            }
            b => panic!("unexpected byte {:?}", b),
        })
        .collect();
    let width = width.expect("didn't find a newline");
    let height = data.len() / width;
    let rect = Rect(Vector2D(width, height));
    Ok(Map {
        tiles: data,
        bounds: rect,
    })
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let map = read_input(input_path)?;
    let tree_count = map
        .traverse(Vector2D(3, 1))
        .filter(|t| matches!(t, Tile::Tree))
        .count();
    Ok(tree_count.to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    let map = read_input(input_path)?;
    let slopes: Vec<Vector2D<usize>> = vec![
        Vector2D(1, 1),
        Vector2D(3, 1),
        Vector2D(5, 1),
        Vector2D(7, 1),
        Vector2D(1, 2),
    ];
    let product: usize = slopes
        .into_iter()
        .map(|slope| {
            map.traverse(slope)
                .filter(|t| matches!(t, Tile::Tree))
                .count()
        })
        .product();
    Ok(product.to_string())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day03part1", || part1(data_path!("day03_input.txt")));
    runner.add("day03part2", || part2(data_path!("day03_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day03_input.txt")).unwrap(), "193");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day03_input.txt")).unwrap(), "1355323200");
    }
}
