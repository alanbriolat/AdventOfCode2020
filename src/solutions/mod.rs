pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

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

    runner
}
