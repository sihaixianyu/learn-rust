pub mod func_demo;
pub mod lifetime_demo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_test() {
        func_demo::test();
    }

    #[test]
    fn lifetime_test() {
        lifetime_demo::test();
    }
}