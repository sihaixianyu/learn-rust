pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

pub fn dyn_summary_demo(case: i32) -> Box<dyn Summary> {
    let post = Post {
        title: "Post title".to_string(),
        author: "Post author".to_string(),
        content: "Post content".to_string(),
    };

    let weibo = Weibo {
        username: "Weibo".to_string(),
        content: "Weibo content".to_string(),
    };

    if case > 0 {
        Box::new(post)
    } else {
        Box::new(weibo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
        let weibo = Weibo{username: "Sunface".to_string(),content: "好像微博没Tweet好用".to_string()};
    
        println!("{}",post.summarize());
        println!("{}",weibo.summarize());
    }

    #[test]
    fn test_case2() {    
        println!("{}",dyn_summary_demo(0).summarize());
        println!("{}",dyn_summary_demo(1).summarize());
    }
}
