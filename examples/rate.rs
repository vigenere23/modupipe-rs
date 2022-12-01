use std::time::Instant;

use modupipe::{
    extractor::Extractor,
    loader::Loader,
    runner::{InfiniteRunner, Pipeline, Runner},
};

struct ConstantExtractor {}

impl Extractor for ConstantExtractor {
    type Output = u32;

    fn get_next(&mut self) -> Self::Output {
        32
    }
}

struct DataRate {
    start: Instant,
    nb_calls: u64,
}

impl DataRate {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            nb_calls: 0,
        }
    }
}

impl<T> Loader<T> for DataRate {
    fn load(&mut self, _: T) {
        let end = Instant::now();
        self.nb_calls += 1;

        let duration = (end - self.start).as_secs_f32();

        if duration >= 1.0 {
            println!("Rate/s : {}", self.nb_calls);
            self.nb_calls = 0;
            self.start = end;
        }
    }
}

pub fn main() {
    let extractor = ConstantExtractor {};
    let loader = DataRate::new();

    let mut runner = InfiniteRunner::new(Pipeline::new(extractor, loader));

    runner.run();
}
