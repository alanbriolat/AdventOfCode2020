pub mod day01;

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::new();

    day01::register(&mut runner);

    runner
}
