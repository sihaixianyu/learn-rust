use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    str,
};

pub fn client_demo() -> io::Result<()> {
    let mut buf = [0; 1024];
    let mut cli_skt = TcpStream::connect("localhost:8888")?;

    cli_skt.write_all(b"This is from client!")?;
    cli_skt.flush()?;

    let n = cli_skt.read(&mut buf)?;
    let s = match str::from_utf8(&buf[..n]) {
        Ok(v) => v,
        Err(e) => {
            panic!("Invalid UTF-8 sequence: {}", e);
        }
    };
    println!("{}", s);

    Ok(())
}

pub fn server_demo() -> io::Result<()> {
    let mut buf = [0; 1024];
    let server = TcpListener::bind("localhost:8888")?;
    let (mut svr_skt, _) = server.accept()?;

    let n = svr_skt.read(&mut buf)?;
    let s = match str::from_utf8(&buf[..n]) {
        Ok(v) => v,
        Err(e) => {
            panic!("Invalid UTF-8 sequence: {}", e);
        }
    };
    println!("{}", s);

    svr_skt.write_all(b"This is from server!")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_demo() -> io::Result<()> {
        client_demo()
    }

    #[test]
    fn test_server_demo() -> io::Result<()> {
        server_demo()
    }
}
