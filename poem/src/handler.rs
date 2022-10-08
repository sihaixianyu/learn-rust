use poem::{error::NotFoundError, handler, Result};

#[handler]
fn return_str() -> &'static str {
    "hello"
}

#[handler]
fn return_err() -> Result<&'static str, NotFoundError> {
    Err(NotFoundError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{http::StatusCode, test::TestClient};

    #[tokio::test]
    async fn test_return_str() {
        let resp = TestClient::new(return_str).get("/").send().await;
        resp.assert_status_is_ok();
        resp.assert_text("hello").await;
    }

    #[tokio::test]
    async fn test_return_err() {
        let resp = TestClient::new(return_err).get("/").send().await;
        resp.assert_status(StatusCode::NOT_FOUND);
    }
}
