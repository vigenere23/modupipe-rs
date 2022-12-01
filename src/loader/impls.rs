use std::sync::mpsc::Sender;

use super::Loader;

pub struct PushToQueue<T> {
    sender: Sender<T>,
}

impl<T> PushToQueue<T> {
    pub fn new(sender: Sender<T>) -> Self {
        Self { sender }
    }
}

impl<T> Loader<T> for PushToQueue<T> {
    fn load(&mut self, item: T) {
        self.sender.send(item).unwrap();
    }
}

pub struct ListLoader<T: Clone> {
    loaders: Vec<Box<dyn Loader<T>>>,
}

impl<T: Clone> ListLoader<T> {
    pub fn new(loaders: Vec<Box<dyn Loader<T>>>) -> Self {
        Self { loaders }
    }
}

impl<'a, T: Clone + 'a> Loader<T> for ListLoader<T> {
    fn load(&mut self, item: T) {
        for loader in self.loaders.iter_mut() {
            loader.load(item.clone())
        }
    }
}
