use std::sync::mpsc::Receiver;

use crate::mapper::Mapper;

use super::Extractor;

pub struct MappedExtractor<Input, Output, E, M>
where
    E: Extractor<Output = Input>,
    M: Mapper<Input, Output = Output>,
{
    extractor: E,
    mapper: M,
}

impl<Input, Output, E, M> Extractor for MappedExtractor<Input, Output, E, M>
where
    E: Extractor<Output = Input>,
    M: Mapper<Input, Output = Output>,
{
    type Output = Output;

    fn get_next(&mut self) -> Self::Output {
        self.mapper.map_next(&mut self.extractor)
    }
}

pub trait MappeableExtractor<E>
where
    E: Extractor<Output = Self::Output>,
{
    type Output;

    fn map<NextOutput, M>(self, mapper: M) -> MappedExtractor<Self::Output, NextOutput, E, M>
    where
        M: Mapper<Self::Output, Output = NextOutput>;
}

impl<E: Extractor> MappeableExtractor<E> for E {
    type Output = E::Output;

    fn map<NextOutput, M>(self, mapper: M) -> MappedExtractor<Self::Output, NextOutput, E, M>
    where
        M: Mapper<Self::Output, Output = NextOutput>,
    {
        MappedExtractor {
            extractor: self,
            mapper,
        }
    }
}

pub struct GetFromQueue<T> {
    receiver: Receiver<T>,
}

impl<T> GetFromQueue<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        Self { receiver }
    }
}

impl<T> Extractor for GetFromQueue<T> {
    type Output = T;

    fn get_next(&mut self) -> Self::Output {
        self.receiver.recv().unwrap()
    }
}
