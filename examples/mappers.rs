use modupipe_rs::{
    extractor::{Extractor, MappeableExtractor},
    mapper::Mapper,
};

struct SimpleExtractor {}

impl Extractor for SimpleExtractor {
    type Output = u32;

    fn get_next(&mut self) -> Self::Output {
        2
    }
}

struct Exponential {
    exponent: u16,
}

impl Exponential {
    pub fn new() -> Self {
        Self { exponent: 0 }
    }
}

impl Mapper<u32> for Exponential {
    type Output = u64;

    fn map_next<E>(&mut self, extractor: &mut E) -> Self::Output
    where
        E: Extractor<Output = u32>,
    {
        let value: u64 = extractor.get_next().into();
        let mapped_value = value.pow(self.exponent.into());
        self.exponent += 1;

        mapped_value
    }
}

fn main() {
    let mut extractor = SimpleExtractor {}.map(Exponential::new());

    for _ in 0..10 {
        println!("VALUE : {}", extractor.get_next());
    }
}
