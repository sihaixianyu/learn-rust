use std::{fs::File, io::Result, io::Read};

pub fn read_file() -> Result<()> {
    let mut file = File::open("Cargo.toml")?;
    let mut buf = String::new();

    file.read_to_string(&mut buf)?;
    println!("{}", buf);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        match read_file() {
            Ok(_) => println!("Ok"),
            Err(err) => println!("{}", err),
        }
    }
}
