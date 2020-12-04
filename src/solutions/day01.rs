use super::prelude::*;

fn read_input(input_path: PathBuf) -> crate::Result<Vec<i64>> {
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);
    // TODO: better error handling
    let data: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    Ok(data)
}

pub fn part1(input_path: PathBuf) -> crate::Result<String> {
    let data = read_input(input_path)?;

    // Really naive O(N^2) implementation
    for (i, x) in data[..(data.len() - 1)].iter().enumerate() {
        for y in &data[i..] {
            if x + y == 2020 {
                return Ok((x * y).to_string());
            }
        }
    }

    return Err("No solution found".into());
}

fn part2_naive(data: &[i64]) -> Option<i64> {
    for x in data.iter() {
        for y in data.iter() {
            for z in data.iter() {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}

fn part2_triangle_enumerate(data: &[i64]) -> Option<i64> {
    for (i, x) in data[..(data.len() - 2)].iter().enumerate() {
        for (j, y) in data[(i + 1)..(data.len() - 1)].iter().enumerate() {
            for z in &data[(i + j + 1)..] {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}

fn part2_triangle_index(data: &[i64]) -> Option<i64> {
    for i in 0..(data.len() - 2) {
        for j in (i + 1)..(data.len() - 1) {
            for k in (j + 1)..data.len() {
                if data[i] + data[j] + data[k] == 2020 {
                    return Some(data[i] * data[j] * data[k]);
                }
            }
        }
    }
    None
}

pub fn part2<F: Fn(&[i64]) -> Option<i64>>(
    input_path: PathBuf,
    implementation: F,
) -> crate::Result<String> {
    let data = read_input(input_path)?;
    match implementation(data.as_ref()) {
        Some(product) => Ok(product.to_string()),
        None => Err("No solution found".into()),
    }
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day01part1", || part1(data_path!("day01_input.txt")));
    runner.add("day01part2_naive", || {
        part2(data_path!("day01_input.txt"), part2_naive)
    });
    runner.add("day01part2_triangle_enumerate", || {
        part2(data_path!("day01_input.txt"), part2_triangle_enumerate)
    });
    runner.add("day01part2_triangle_index", || {
        part2(data_path!("day01_input.txt"), part2_triangle_index)
    });
    // runner.add("day01part2_sorted_naive", || part2(data_path!("day01_input_sorted.txt"), part2_naive));
    // runner.add("day01part2_sorted_triangle_enumerate", || part2(data_path!("day01_input_sorted.txt"), part2_triangle_enumerate));
    runner.add("day01part2_sorted_triangle_index", || {
        part2(data_path!("day01_input_sorted.txt"), part2_triangle_index)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day01_input.txt")).unwrap(), "357504");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(data_path!("day01_input.txt"), part2_naive).unwrap(),
            "12747392"
        );
        assert_eq!(
            part2(data_path!("day01_input.txt"), part2_triangle_enumerate).unwrap(),
            "12747392"
        );
        assert_eq!(
            part2(data_path!("day01_input.txt"), part2_triangle_index).unwrap(),
            "12747392"
        );
    }
}
