#[derive(PartialEq, Debug)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn test() {
    let v1 = vec![1, 2, 3];

    for val in v1.iter() {
        println!("{}", val)
    }
}

pub fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}