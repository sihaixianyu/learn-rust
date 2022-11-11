pub fn demo1() {
    let x = Some(true);
    let y = true;

    match x {
        Some(x) if x => println!("In branch 1"),
        Some(_) if y => println!("In branch 2"),
        _ => println!("In branch 3")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo1() {
        demo1();
    }
}