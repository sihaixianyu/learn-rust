use iterator_demo::Counter;
use iterator_demo::Shoe;

mod closure_demo;
mod iterator_demo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closure_test() {
        closure_demo::test();
    }

    #[test]
    fn iterator_test() {
        iterator_demo::test()
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4])
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = iterator_demo::shoe_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }

    #[test]
    fn call_next_directly() {
        let mut cnt = Counter::new();

        assert_eq!(cnt.next(), Some(1));
        assert_eq!(cnt.next(), Some(2));
        assert_eq!(cnt.next(), Some(3));
        assert_eq!(cnt.next(), Some(4));
        assert_eq!(cnt.next(), Some(5));
        assert_eq!(cnt.next(), None);
    }

    #[test]
    fn use_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(sum, 18);
    }
}