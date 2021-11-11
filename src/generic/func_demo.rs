// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn test() {
    let mut num_list = vec![34, 50, 40, 100, 65];
    let mut res = largest(&num_list);
    println!("The largest num is {}", res);

    num_list = vec![12, 40, 70, 120, 55];
    res = largest(&num_list);
    println!("The largest num is {}", res);
}

// pub fn test() {
//     let p: Point<f32> = Point{x: 1., y: 5.};
//     println!("{}", p.x);
//     println!("{}", p.distance_from_origin())
// }