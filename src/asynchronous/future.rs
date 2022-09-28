pub async fn hello_world() {
    hello_cat().await;
    println!("hello, world!");
}

pub async fn hello_cat() {
    println!("hello, cat!");
}

pub struct Song {
    author: String,
    name: String,
}

pub async fn learn_song() -> Song {
    Song {
        author: "蔡徐坤".to_string(),
        name: "鸡你太美".to_string(),
    }
}

pub async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的《{}》 ~ {}",
        song.author, song.name, "鸡你太美, Baby"
    )
}

pub async fn dance() {
    println!("运球, 铁山靠, 千鸟, 螺旋丸")
}

pub async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

pub async fn sing_and_dance() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f2, f1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn test_hello_cat() {
        let future = hello_cat();
        println!("[Log] hello_world not execute here");
        block_on(future);
        println!("[Log] hello_world execute aftere block_on");
    }

    #[test]
    fn test_hello_world() {
        let future = hello_world();
        block_on(future);
    }

    #[test]
    fn test_sing_and_dance() {
        block_on(sing_and_dance());
    }
}
