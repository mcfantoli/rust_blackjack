use std::fmt;
use std::cmp::Ordering;
use std::slice::Iter;
use crate::cards::Rank::*;
use crate::cards::Suit::*;


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

    pub fn ordinal(&self) -> usize {
        match *self {
            Two => 0,
            Three => 1,
            Four => 2,
            Five => 3,
            Six => 4,
            Seven => 5,
            Eight => 6,
            Nine => 7,
            Ten => 8,
            Jack => 9,
            Queen => 10,
            King => 11,
            Ace => 12,
        }
    }

    pub fn ranks() -> &'static [Rank] {
        static RANKS: [Rank; 13] = [ Two, Three, Four, Five,
            Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace ];
        &RANKS[..]
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "10",
            Jack => "J",
            Queen => "Q",
            King => "K",
            Ace => "A",
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Debug)]
pub enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds,
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        Suit::suits().into_iter()
    }

    pub fn suits() -> &'static [Suit] {
        static SUITS: [Suit; 4] = [ Hearts, Spades, Clubs, Diamonds ];
        &SUITS[..]
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            Hearts => "&#9829;",
            Spades => "&#9824;",
            Clubs => "&#9827;",
            Diamonds => "&#9830;",
        }
    }
}

// Struct for Card. Each card has a rank (1 to Ace) and
// a suit (Spades, Clubs, Hearts, Diamonds)
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    // Display card
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write formatted string to buffer f
        write!(f, "{}", self.to_str())
    }
}

impl Ord for Card {
    // Compare rank
    fn cmp(&self, other: &Card) -> Ordering {
        self.compare_rank(other)
    }
}

impl Card {
    pub fn new (rank: Rank, suit: Suit) -> Card {
        Card {
            rank,
            suit,
        }
    }

    pub fn compare_rank(&self, other: &Card) -> Ordering {
        if self.rank.ordinal() > other.rank.ordinal() {
            return Ordering::Less;
        }
        else if self.rank.ordinal() < other.rank.ordinal() {
            return Ordering::Greater;
        }
        else {
            return Ordering::Equal;
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}{}", self.rank.to_str(), self.suit.to_str())
    }
}