pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

mod prelude;

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::new();

    day01::register(&mut runner);
    day02::register(&mut runner);
    day03::register(&mut runner);
    day04::register(&mut runner);
    day05::register(&mut runner);
    day06::register(&mut runner);
    day07::register(&mut runner);
    day08::register(&mut runner);
    day09::register(&mut runner);
    day10::register(&mut runner);
    day11::register(&mut runner);
    day12::register(&mut runner);
    day13::register(&mut runner);
    day14::register(&mut runner);
    day15::register(&mut runner);
    day16::register(&mut runner);
    day17::register(&mut runner);
    day18::register(&mut runner);
    // day19::register(&mut runner);
    // day20::register(&mut runner);
    // day21::register(&mut runner);
    day22::register(&mut runner);
    // day23::register(&mut runner);
    // day24::register(&mut runner);
    // day25::register(&mut runner);

    runner
}
