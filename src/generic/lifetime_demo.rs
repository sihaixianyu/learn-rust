pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn test() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let res = longest(s1.as_str(), s2);
    println!("The longest string is {}", res);

    let novel = ImportantExcerpt {
        part: &s1,
    };

    println!("{}", novel.announce("Hello"));
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}