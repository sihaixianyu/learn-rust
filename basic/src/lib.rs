pub mod asyncs;
pub mod closure;
pub mod collection;
pub mod io;
pub mod keyword;
pub mod lifetime;
pub mod net;
pub mod pointer;

#[cfg(test)]
mod tests {

    #[test]
    fn test_case1() {
        println!("Hello!")
    }
}
