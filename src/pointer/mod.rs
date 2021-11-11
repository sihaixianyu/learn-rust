mod box_demo;
mod deref_demo;
mod drop_demo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_demo() {
        box_demo::run();
    }

    #[test]
    fn test_deref_demo() {
        deref_demo::run();
    }

    #[test]
    fn test_drop_demo() {
        drop_demo::run();
    }
}