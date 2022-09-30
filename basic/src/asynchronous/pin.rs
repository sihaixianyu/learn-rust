use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
pub struct UnpinDemo {
    a: String,
    b: *const String,
}

impl UnpinDemo {
    pub fn new(txt: &str) -> Self {
        UnpinDemo {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    pub fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    pub fn a(&self) -> &str {
        &self.a
    }

    pub fn b(&self) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.b) }
    }
}

#[derive(Debug)]
pub struct PinDemo {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl PinDemo {
    pub fn new(txt: &str) -> Pin<Box<Self>> {
        let pin_demo = PinDemo {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };

        let mut boxed = Box::pin(pin_demo);
        let self_ptr: *const String = &boxed.as_ref().a;

        unsafe {
            boxed.as_mut().get_unchecked_mut().b = self_ptr;
        };

        boxed
    }

    pub fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    pub fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_unpin_demo() {
        let mut test1 = UnpinDemo::new("test1");
        test1.init();
        let mut test2 = UnpinDemo::new("test2");
        test2.init();

        // Exchange addrs between test1 and test2
        mem::swap(&mut test1, &mut test2);

        println!("a: {}, b: {}", test1.a(), test1.b());
        println!("a: {}, b: {}", test2.a(), test2.b());
    }

    #[test]
    fn test_pin_demo() {
        let mut test1 = PinDemo::new("test1");
        let mut test2 = PinDemo::new("test2");

        // Exchange addrs between test1 and test2
        mem::swap(&mut test1, &mut test2);

        println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
        println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
    }
}
