use aoc2020::solutions;

fn main() {
    let runner = solutions::build_runner();
    runner.run_all().for_each(|(name, result)| match result {
        Ok(output) => println!("{}: {}", name, output),
        Err(err) => println!("ERROR: {}: {}", name, err),
    });
}
