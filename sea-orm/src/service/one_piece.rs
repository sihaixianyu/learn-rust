use sea_orm::*;

use crate::entity::one_piece::Model;
use crate::entity::{prelude::*, *};

pub async fn insert(pool: &DbConn) -> Result<(), DbErr> {
    let naval = one_piece::ActiveModel {
        name: ActiveValue::Set("黄猿".to_string()),
        age: ActiveValue::Set(50),
        reward: ActiveValue::Set(30),
        ..Default::default() // Use Default()::default() func to omit other fileds.
    };

    if let Err(err) = OnePiece::insert(naval).exec(pool).await {
        panic!("{}", err)
    }

    Ok(())
}

pub async fn find(pool: &DbConn) -> Result<Vec<Model>, DbErr> {
    let roles = OnePiece::find().all(pool).await.unwrap();
    Ok(roles)
}

pub async fn update(pool: &DbConn) -> Result<(), DbErr> {
    let naval = one_piece::ActiveModel {
        id: ActiveValue::Set(15), // PK requires to be provided while executing update
        name: ActiveValue::Set("黄猿".to_string()),
        age: ActiveValue::Set(50),
        reward: ActiveValue::Set(35),
        ..Default::default()
    };

    OnePiece::update(naval).exec(pool).await?;
    Ok(())
}

pub async fn delete(pool: &DbConn) -> Result<(), DbErr> {
    OnePiece::delete_by_id(15).exec(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::service::db_service;

    use super::*;

    #[tokio::test]
    async fn test_insert() {
        let pool = db_service::init_conn().await.unwrap();

        if let Err(err) = insert(&pool).await {
            match err {
                DbErr::Exec(err) => {
                    println!("Please check your input carefully!");
                    println!("Info => {{{}}}", err)
                }
                _ => panic!("{}", err),
            }
        }
    }
}
