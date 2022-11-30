use crate::mapper::base::Mapper;

pub trait Extractor {
    type Output;

    fn get_next(&mut self) -> Self::Output;
}

pub struct MappedExtractor<Input, Output, E, M>
where
    E: Extractor<Output = Input>,
    M: Mapper<Input = Input, Output = Output>,
{
    extractor: E,
    mapper: M,
}

impl<Input, Output, E, M> Extractor for MappedExtractor<Input, Output, E, M>
where
    E: Extractor<Output = Input>,
    M: Mapper<Input = Input, Output = Output>,
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
        M: Mapper<Input = Self::Output, Output = NextOutput>;
}

impl<E: Extractor> MappeableExtractor<E> for E {
    type Output = E::Output;

    fn map<NextOutput, M>(self, mapper: M) -> MappedExtractor<Self::Output, NextOutput, E, M>
    where
        M: Mapper<Input = Self::Output, Output = NextOutput>,
    {
        MappedExtractor {
            extractor: self,
            mapper,
        }
    }
}
