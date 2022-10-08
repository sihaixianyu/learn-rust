#[cfg(test)]
mod tests {
    use mini_redis::{client};

    #[tokio::test]
    async fn test_hello(){
        let mut cli = client::connect("localhost:6379").await.unwrap();
        cli.set("msg", "hello".into()).await.unwrap();

        let res = cli.get("msg").await.unwrap();
        println!("[GET] result={:?}", res);
    }
}
