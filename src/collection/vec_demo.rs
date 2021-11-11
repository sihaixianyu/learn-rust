pub fn test() {
    let mut vec = vec![1, 2, 3];
    vec.push(10);

    let opt = vec.get(0);
    match opt {
        Some(x) => println!("{}", x),
        None => println!("There is no value"),
    }
}