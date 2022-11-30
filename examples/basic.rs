use modupipe_rs::extractor::Extractor;

struct Fibonacci {
    i: u32,
    first: u32,
    second: u32,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self {
            i: 0,
            first: 0,
            second: 1,
        }
    }
}

impl Extractor for Fibonacci {
    type Output = u32;

    fn get_next(&mut self) -> Self::Output {
        self.i += 1;

        if self.i <= 2 {
            return self.i - 1;
        }

        let value = self.first + self.second;
        self.first = self.second;
        self.second = value;

        value
    }
}

fn main() {
    let mut extractor = Fibonacci::new();

    for _ in 0..10 {
        println!("VALUE : {}", extractor.get_next());
    }
}
