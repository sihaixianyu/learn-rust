pub mod collection;
pub mod io;
pub mod net;
pub mod keyword;
pub mod lifetime;
pub mod closure;

#[cfg(test)]
mod tests {

    #[test]
    fn test_case1() {
        println!("Hello!")
    }
}
