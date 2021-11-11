pub mod map_demo;
pub mod vec_demo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        map_demo::test();
    }

    #[test]
    fn vec_test() {
        vec_demo::test();
    }
}