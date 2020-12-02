pub mod day01;
pub mod day02;

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::new();

    day01::register(&mut runner);
    day02::register(&mut runner);

    runner
}
