use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Queue<T> {
    pub receiver: Receiver<T>,
    pub sender: Sender<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let (sender, receiver) = channel();

        Self { receiver, sender }
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}
