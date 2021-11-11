pub mod panic_demo;
pub mod recover_demo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panic_test() {
        panic_demo::test();
    }

    #[test]
    fn recover_test() {
        let _res = recover_demo::test();
    }
}