#[derive(Debug)]
pub struct SelfRef<'a> {
    pub val: String,
    // 该引用指向上面的value
    pub ptr_to_val: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let s = "aaa".to_string();
        let v = SelfRef {
            // val: s,
            val: s.clone(),
            ptr_to_val: &s,
        };
        println!("{:?}", v);
    }
}
