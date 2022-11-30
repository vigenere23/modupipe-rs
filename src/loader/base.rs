pub trait Loader<Input> {
    fn load(&mut self, item: Input);
}
