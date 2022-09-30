pub fn exec<F: FnOnce()>(f: F) {
    f()
}

pub fn exec1<F: FnMut()>(mut f: F) {
    f()
}

pub fn exec2<F: Fn()>(f: F) {
    f()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let s = String::new();
        let update_string = || println!("{}", s);

        exec(update_string);
        exec1(update_string);
        exec2(update_string);
    }

    #[test]
    fn test_case2() {
        let mut s = String::new();
        let _update_string = |str| -> String {
            s.push_str(str);
            s
        };

        // exec(update_string("Hello"));
        // exec1(update_string("Hello"));
        // exec2(update_string("Hello"));
    }
}
