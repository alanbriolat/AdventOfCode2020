use std::collections::BTreeMap;

pub type SolutionResult = crate::Result<String>;

pub trait SolutionFn: Fn() -> SolutionResult {}

impl<F: Fn() -> SolutionResult> SolutionFn for F {}

pub trait Solution {
    fn run(&self) -> SolutionResult;
}

impl<F: SolutionFn> Solution for F {
    fn run(&self) -> SolutionResult {
        self()
    }
}

pub struct Runner {
    solutions: BTreeMap<&'static str, Box<dyn Solution>>,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            solutions: BTreeMap::new(),
        }
    }

    pub fn add<S: Solution + 'static>(&mut self, name: &'static str, solution: S) {
        if self.solutions.contains_key(name) {
            panic!("solution {:?} already exists", name);
        }
        self.solutions.insert(name, Box::new(solution));
    }

    pub fn list(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.solutions.keys().cloned()
    }

    pub fn run_all(&self) -> impl Iterator<Item = (&str, SolutionResult)> {
        self.solutions.iter().map(|(&name, solution)| (name, solution.run()))
    }

    pub fn run(&self, name: &str) -> SolutionResult {
        match self.solutions.get(&name) {
            Some(solution) => solution.run(),
            None => panic!("no solution {:?}", name),
        }
    }
}
