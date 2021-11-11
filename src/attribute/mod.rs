pub mod cfg_demo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfg_test() {
        cfg_demo::test();
    }
}