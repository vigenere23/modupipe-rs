use crate::{extractor::Extractor, loader::Loader};

use super::base::Runner;

pub struct Pipeline<T, E, L>
where
    E: Extractor<Output = T>,
    L: Loader<T>,
{
    extractor: E,
    loader: L,
}

impl<T, E, L> Pipeline<T, E, L>
where
    E: Extractor<Output = T>,
    L: Loader<T>,
{
    pub fn new(extractor: E, loader: L) -> Self {
        Self { extractor, loader }
    }
}

impl<T, E, L> Runner for Pipeline<T, E, L>
where
    E: Extractor<Output = T>,
    L: Loader<T>,
{
    fn run(&mut self) {
        self.loader.load(self.extractor.get_next());
    }
}

pub struct RangeRunner<R: Runner> {
    runner: R,
    nb_times: u32,
}

impl<R: Runner> RangeRunner<R> {
    pub fn new(runner: R, nb_times: u32) -> Self {
        Self { runner, nb_times }
    }
}

impl<R: Runner> Runner for RangeRunner<R> {
    fn run(&mut self) {
        for _ in 0..self.nb_times {
            self.runner.run();
        }
    }
}
