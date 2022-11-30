use modupipe_rs::{
    extractor::{Extractor, MappeableExtractor},
    loader::Loader,
    mapper::Mapper,
    runner::{Pipeline, RangeRunner, Runner},
};

struct IncreasingExtractor {
    value: u32,
}

impl IncreasingExtractor {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Extractor for IncreasingExtractor {
    type Output = u32;

    fn get_next(&mut self) -> Self::Output {
        let value = self.value;
        self.value += 1;

        value
    }
}

struct Double {}

impl Mapper<u32> for Double {
    type Output = u64;

    fn map_next<E>(&mut self, extractor: &mut E) -> Self::Output
    where
        E: Extractor<Output = u32>,
    {
        let value: u64 = extractor.get_next().into();
        value * 2
    }
}

struct ConsoleLogger {}

impl<Input: ToString> Loader<Input> for ConsoleLogger {
    fn load(&mut self, item: Input) {
        println!("VALUE : {}", item.to_string());
    }
}

fn main() {
    let extractor = IncreasingExtractor::new().map(Double {});
    let loader = ConsoleLogger {};
    let pipeline = Pipeline::new(extractor, loader);

    let mut runner = RangeRunner::new(pipeline, 10);

    runner.run();
}
