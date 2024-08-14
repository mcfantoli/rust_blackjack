use std::fmt;
use std::cmp::Ordering;
use std::slice::Iter;


#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Ord for Rank {
    fn cmp(&self, other: &Rank) -> Ordering {
        let self_ord = self.ordinal();
        let other_ord = other.ordinal();

        if self_ord > other_ord {
            return Ordering::Less;
        }
        else if self_ord < other_ord {
            return Ordering::Greater;
        }
        else {
            return Ordering::Equal;
        }
    }
}

impl Rank {
    pub fn iterator() -> Iter<'static, Rank> {
        Rank::ranks().into_iter()
    }
}

// Struct for Card. Each card has a rank (1 to Ace) and
// a suit (Spades, Clubs, Hearts, Diamonds)
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    // Display card
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write formatted string to buffer f
        write!(f, "{}", self.to_str());
    }
}

impl Ord for Card {
    // Compare rank
    fn cmp(&self, other: &Card) -> Ordering {
        self.compare_rank(other);
    }
}

impl Card {
    pub fn new (rank: Rank, suit: Suit) -> Card {
        Card {
            rank,
            suit,
        }
    }
}