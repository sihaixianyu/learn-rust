pub mod asynchronous;
pub mod closure;
pub mod collection;
pub mod io;
pub mod keyword;
pub mod lifetime;
pub mod net;

#[cfg(test)]
mod tests {

    #[test]
    fn test_case1() {
        println!("Hello!")
    }
}
