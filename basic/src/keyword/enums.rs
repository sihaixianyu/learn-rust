#[derive(Debug)]
pub enum PokerSuit {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

pub fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let card1 = PokerSuit::Hearts(5);
        let card2 = PokerSuit::Diamonds(13);

        print_suit(card1);
        print_suit(card2);
    }
}
