#[cfg(test)]
mod tests {
    use axum::{Router, routing::get};

    #[tokio::test]
    async fn test_handler() {
        // build our application with a single route
        let app = Router::new().route("/", get(|| async { "Hello, World!" }));

        // run it with hyper on localhost:3000
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
