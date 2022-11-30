use derive_new::new;

trait Extractor {
    type Output;

    fn get_next(&mut self) -> Self::Output;
    // fn map<NextOutput>(
    //     &mut self,
    //     mapper: &mut dyn Mapper<Input = Self::Output, Output = NextOutput>,
    // ) -> dyn Extractor<Output = NextOutput>;
}

#[derive(new)]
struct IntExtractor {}

impl Extractor for IntExtractor {
    type Output = u32;

    fn get_next(&mut self) -> Self::Output {
        32
    }
}

trait Mapper {
    type Input;
    type Output;

    fn map_next<E>(&mut self, extractor: &mut E) -> Self::Output
    where
        E: Extractor<Output = Self::Input>;
}

#[derive(new)]
struct DoubleMapper {}

impl Mapper for DoubleMapper {
    type Input = u32;
    type Output = u32;

    fn map_next<E>(&mut self, extractor: &mut E) -> Self::Output
    where
        E: Extractor<Output = Self::Input>,
    {
        extractor.get_next() * 2
    }
}

struct MappedExtractor<Input, Output, E, M>
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

trait MappeableExtractor<E>
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

fn main() {
    let mut extractor = IntExtractor::new().map(DoubleMapper::new());

    println!("VALUE : {:?}", extractor.get_next());
}
