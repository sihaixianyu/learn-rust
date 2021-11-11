use std::thread;
use std::time::Duration;

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn test() {
    let intensity = 26;
    let random_num = 7;

    generate_workout(intensity, random_num);
}

fn generate_workout(intensity: u32, random_num: u32) {
    // let expensive_res = simulated_expensive_calculation(intensity);
    let mut cacher = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", cacher.value(intensity));
        println!("Next, do {} situps", cacher.value(intensity));
    } else {
        if random_num == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                cacher.value(intensity)
            );
        }
    }
}