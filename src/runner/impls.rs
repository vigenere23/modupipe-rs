use std::{
    sync::{Arc, Mutex},
    thread,
};

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

unsafe impl<T, E, L> Send for Pipeline<T, E, L>
where
    E: Extractor<Output = T>,
    L: Loader<T>,
{
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
        let item = self.extractor.get_next();
        self.loader.load(item);
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

pub struct InfiniteRunner<R: Runner> {
    runner: R,
}

impl<R: Runner> InfiniteRunner<R> {
    pub fn new(runner: R) -> Self {
        Self { runner }
    }
}

impl<R: Runner> Runner for InfiniteRunner<R> {
    fn run(&mut self) {
        loop {
            self.runner.run();
        }
    }
}

pub struct ConcurrentRunner {
    runners: Vec<Arc<Mutex<dyn Runner + Send>>>,
}

unsafe impl Send for ConcurrentRunner {}

impl ConcurrentRunner {
    pub fn new(runners: Vec<Arc<Mutex<dyn Runner + Send>>>) -> Self {
        Self { runners }
    }
}

impl Runner for ConcurrentRunner {
    fn run(&mut self) {
        let threads: Vec<thread::JoinHandle<()>> = self
            .runners
            .iter()
            .cloned()
            .map(|runner| {
                thread::spawn(move || {
                    runner.lock().unwrap().run();
                })
            })
            .collect();

        for thread in threads {
            thread.join().unwrap();
        }
    }
}
