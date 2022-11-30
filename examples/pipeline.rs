use modupipe_rs::{
    extractor::Extractor,
    loader::Loader,
    runner::{Pipeline, RangeRunner, Runner},
};

struct IncrementalExtractor {
    i: u32,
}

impl IncrementalExtractor {
    pub fn new() -> Self {
        Self { i: 0 }
    }
}

impl Extractor for IncrementalExtractor {
    type Output = u32;

    fn get_next(&mut self) -> Self::Output {
        let value = self.i;
        self.i += 1;

        value
    }
}

struct ConsoleLogger {
    i: u32,
}

impl ConsoleLogger {
    pub fn new() -> Self {
        Self { i: 0 }
    }
}

impl<Input: ToString> Loader<Input> for ConsoleLogger {
    fn load(&mut self, item: Input) {
        println!("VALUE {} : {}", self.i, item.to_string());

        self.i += 1;
    }
}

fn main() {
    let extractor = IncrementalExtractor::new();
    let loader = ConsoleLogger::new();
    let pipeline = Pipeline::new(extractor, loader);

    let mut runner = RangeRunner::new(pipeline, 10);

    runner.run();
}
