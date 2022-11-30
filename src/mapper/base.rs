use crate::extractor::base::Extractor;

pub trait Mapper {
    type Input;
    type Output;

    fn map_next<E>(&mut self, extractor: &mut E) -> Self::Output
    where
        E: Extractor<Output = Self::Input>;
}
