use std::env;
use std::time;

use aoc2020::solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let runner = solutions::build_runner();
    let names: Vec<_> = match args.get(1) {
        Some(filter) => runner.list().filter(|name| name.contains(filter)).collect(),
        None => runner.list().collect(),
    };

    for name in names.iter() {
        let start = time::Instant::now();
        let result = runner.run(name);
        let elapsed = time::Instant::now().duration_since(start);
        match result {
            Ok(output) => println!("{}: {} ({:?})", name, output, elapsed),
            Err(err) => println!("ERROR: {}: {}", name, err),
        }
    }
}
