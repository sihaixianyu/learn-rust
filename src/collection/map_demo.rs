use std::collections::HashMap;

pub fn test() {
    let mut map = HashMap::new();

    map.insert("Blue", 10);

    let opt = map.get("Blue");
    match opt {
        Some(x) => println!("{}", x),
        None => println!("There is no value"),
    }
}
