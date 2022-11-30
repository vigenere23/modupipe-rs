use modupipe_rs::{
    extractor::{Extractor, MappeableExtractor},
    mapper::Mapper,
};

struct RawPayload {
    pub a: String,
    pub b: u32,
}

struct PayloadExtractor {}

impl Extractor for PayloadExtractor {
    type Output = RawPayload;

    fn get_next(&mut self) -> Self::Output {
        RawPayload {
            a: "Hello, world".to_string(),
            b: 64,
        }
    }
}

struct ParsedPayload {
    pub value: String,
}

struct PayloadParser {}

impl Mapper<RawPayload> for PayloadParser {
    type Output = ParsedPayload;

    fn map_next<E>(&mut self, extractor: &mut E) -> Self::Output
    where
        E: Extractor<Output = RawPayload>,
    {
        let value = extractor.get_next();
        let value = format!("{} - {}", value.a, value.b);

        ParsedPayload { value }
    }
}

pub fn main() {
    let mut extractor = PayloadExtractor {}.map(PayloadParser {});

    for _ in 0..10 {
        println!("VALUE : {}", extractor.get_next().value);
    }
}
