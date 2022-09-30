#[derive(Debug)]
pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        Some(self.count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_into_iter() {
        let nums = vec![1, 2, 3];

        for v in nums.into_iter() {
            println!("{:?}", v);
        }
    }

    #[test]
    fn test_iter() {
        let nums = vec![1, 2, 3];

        for v in nums.iter() {
            println!("{:?}", v);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut nums = vec![1, 2, 3];

        for v in nums.iter_mut() {
            *v += 1;
        }

        for v in nums.iter() {
            println!("{:?}", v);
        }
    }

    #[test]
    fn test_enumerate() {
        let nums = vec![1, 2, 3];
        
        for (i, v) in nums.iter().enumerate() {
            println!("idx:{}, val:{}", i, v);
        }
    }

    #[test]
    fn test_consuming_adapter() {
        let nums = vec![1, 2, 3];

        let res: i32 = nums.into_iter().sum();
        println!("{:?}", res);
    }

    #[test]
    fn test_iterable_adapter() {
        let nums = vec![1, 2, 3];

        let res: Vec<i32> = nums.into_iter().map(|x| x + 1).collect();
        println!("{:?}", res);
    }

    #[test]
    fn test_collect() {
        let keys = vec!["k1", "k2", "k3"];
        let vals = vec![1, 2, 3];

        let res: HashMap<&str, i32> = keys.into_iter().zip(vals.into_iter()).collect();
        println!("{:?}", res);
    }

    #[test]
    fn test_counter() {
        let mut cnt = Counter::new();

        for _ in 0..10 {
            cnt.next();
        }

        println!("{:?}", cnt);
    }
}
