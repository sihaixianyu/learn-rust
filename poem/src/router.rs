use poem::{
    error::ParseQueryError,
    handler,
    http::StatusCode,
    web::{Json, Path, Query},
    IntoResponse, Response, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: u8,
}

#[handler]
async fn get_user(id: Path<String>) -> String {
    id.0
}

#[handler]
async fn del_user(id: Path<String>) {
    println!("del {}", id.0)
}

#[handler]
async fn crt_user(res: Result<Query<User>>) -> Result<impl IntoResponse> {
    match res {
        Ok(Query(user)) => Ok(Json(user).into_response()),
        Err(err) if err.is::<ParseQueryError>() => {
            let resp = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(err.to_string());
            Ok(resp)
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{get, listener::TcpListener, post, Route, Server};

    #[tokio::test]
    async fn test_case1() -> Result<(), std::io::Error> {
        let app = Route::new()
            .at("/user/:id", get(get_user).delete(del_user))
            .at("/user", post(crt_user));

        Server::new(TcpListener::bind("localhost:8888"))
            .run(app)
            .await
    }
}
