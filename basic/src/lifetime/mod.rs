pub fn longest<'a, 'b>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
pub struct Foo;

impl Foo {
    pub fn mutate_and_share(&mut self) -> &Self {
        &*self
    }

    pub fn share(&self) {}
}

#[derive(Debug)]
pub struct Point(i32, i32);

impl Point {
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.0 = x;
        self.1 = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let x = "Hello";
        let y = "World";

        let res = longest(x, y);
        assert_eq!(res, "Hello");
    }

    #[test]
    fn test_foo() {
        let mut foo = Foo;
        let loan = foo.mutate_and_share();
        // foo.share();
        println!("{:?}", loan);
    }

    #[test]
    fn test_reborrow() {
        let mut p = Point(0, 0);
        let r = &mut p;
        let rr: &Point = &*r;
    
        println!("{:?}", rr);
        r.move_to(10, 10);
        println!("{:?}", r);
    }
}
