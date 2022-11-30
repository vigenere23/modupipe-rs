pub trait Extractor {
    type Output;

    fn get_next(&mut self) -> Self::Output;
}
