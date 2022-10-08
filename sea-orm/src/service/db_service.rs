use sea_orm::{Database, DbConn, DbErr};

const DATABASE_URL: &str = "mysql://root:486923@localhost:3306/learn_sql";

pub async fn init_conn() -> Result<DbConn, DbErr> {
    let res = Database::connect(DATABASE_URL).await;
    match res {
        Ok(conn) => Ok(conn),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn test_connect() {
        if let Err(err) = block_on(init_conn()) {
            panic!("{}", err);
        }
    }
}
