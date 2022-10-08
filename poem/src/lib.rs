pub mod extractor;
pub mod handler;
pub mod router;

use poem::{handler, web::Path};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    println!("[GET] from {}", name);
    format!("hello: {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{get, listener::TcpListener, Route, Server};

    #[tokio::test]
    async fn test_hello() -> Result<(), std::io::Error> {
        let app = Route::new().at("/hello/:name", get(hello));

        Server::new(TcpListener::bind("127.0.0.1:3000"))
            .run(app)
            .await
    }
}
