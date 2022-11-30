use derive_new::new;
use modupipe_rs::{
    extractor::base::{Extractor, MappeableExtractor},
    mapper::base::Mapper,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

struct RandomExtractor {
    random: ThreadRng,
}

impl RandomExtractor {
    pub fn new() -> Self {
        Self {
            random: thread_rng(),
        }
    }
}

impl Extractor for RandomExtractor {
    type Output = u32;

    fn get_next(&mut self) -> Self::Output {
        self.random.gen()
    }
}

#[derive(new)]
struct Double {}

impl Mapper for Double {
    type Input = u32;
    type Output = u64;

    fn map_next<E>(&mut self, extractor: &mut E) -> Self::Output
    where
        E: Extractor<Output = Self::Input>,
    {
        let value: u64 = extractor.get_next().into();
        value * 2
    }
}

fn main() {
    let mut extractor = RandomExtractor::new().map(Double::new());

    for _ in 1..10 {
        println!("VALUE : {:?}", extractor.get_next());
    }
}
