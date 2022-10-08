use poem::{
    error::ParseQueryError,
    handler,
    http::{Method, StatusCode, Uri},
    web::{Query, RemoteAddr},
    IntoResponse, Response, Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Params {
    name: String,
}

#[handler]
fn index1(remote_addr: &RemoteAddr, method: Method, uri: &Uri) -> Result<impl IntoResponse> {
    let msg = format!("Info: addr={}, method={}, uri={}", remote_addr, method, uri);
    let resp = Response::builder().status(StatusCode::OK).body(msg);
    Ok(resp)
}

#[handler]
fn index2(res: Result<Query<Params>>) -> Result<impl IntoResponse> {
    match res {
        Ok(Query(params)) => Ok(params.name.into_response()),
        Err(err) if err.is::<ParseQueryError>() => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(err.to_string())),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::test::TestClient;

    #[tokio::test]
    async fn test_index1() {
        let resp = TestClient::new(index1).get("/").send().await;
        resp.assert_status_is_ok();
        println!("{:?}", resp.json().await);
    }

    #[tokio::test]
    async fn test_index2() {
        let resp = TestClient::new(index2).get("/?name=ztx").send().await;
        resp.assert_status_is_ok();
        resp.assert_text("ztx").await;
    }
}
