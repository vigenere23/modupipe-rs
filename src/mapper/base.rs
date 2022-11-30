use crate::extractor::Extractor;

pub trait Mapper<Input> {
    type Output;

    fn map_next<E>(&mut self, extractor: &mut E) -> Self::Output
    where
        E: Extractor<Output = Input>;
}
