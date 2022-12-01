use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use modupipe::{
    extractor::{Extractor, GetFromQueue},
    loader::{ListLoader, Loader, PushToQueue},
    runner::{ConcurrentRunner, InfiniteRunner, Pipeline, Runner},
    utils::Queue,
};

struct IncrementalExtractor {
    i: u32,
}

impl IncrementalExtractor {
    pub fn new() -> Self {
        Self { i: 0 }
    }
}

impl Extractor for IncrementalExtractor {
    type Output = u32;

    fn get_next(&mut self) -> Self::Output {
        thread::sleep(Duration::from_millis(200));
        let value = self.i;
        self.i += 1;

        println!("Extracted value: {value}");

        value
    }
}

struct ConsoleLogger {
    pub prefix: String,
}

impl<Input: ToString> Loader<Input> for ConsoleLogger {
    fn load(&mut self, item: Input) {
        println!("[{}] {}", self.prefix, item.to_string());
    }
}

pub fn main() {
    let queue_1: Queue<u32> = Queue::new();
    let queue_2: Queue<u32> = Queue::new();

    let pipeline_push = InfiniteRunner::new(Pipeline::new(
        IncrementalExtractor::new(),
        ListLoader::new(vec![
            Box::from(PushToQueue::new(queue_1.sender.clone())),
            Box::from(PushToQueue::new(queue_2.sender.clone())),
        ]),
    ));

    let pipeline_pull_1 = InfiniteRunner::new(Pipeline::new(
        GetFromQueue::new(queue_1.receiver),
        ConsoleLogger {
            prefix: "Loader 1".into(),
        },
    ));

    let pipeline_pull_2 = InfiniteRunner::new(Pipeline::new(
        GetFromQueue::new(queue_2.receiver),
        ConsoleLogger {
            prefix: "Loader 2".into(),
        },
    ));

    let mut runner = ConcurrentRunner::new(vec![
        Arc::from(Mutex::from(pipeline_pull_1)),
        Arc::from(Mutex::from(pipeline_pull_2)),
        Arc::from(Mutex::from(pipeline_push)),
    ]);
    runner.run();
}
