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

    runner
}
