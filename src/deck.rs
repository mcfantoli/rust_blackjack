mod cards;
use cards::*;
use Rand::rng;


#[derive(Clone)]
pub struct Deck {
    cards: Vec<Card>,
    dealt: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {

        let mut cards: Vec<Card> = vec![];
        for s in Suit::iterator() {
            for r in Rank::iterator() {
                cards.push(Card{rank: *r, suit: *s});
            }
        }

        Deck {cards: cards.clone(), dealt: Vec::with_capacity(cards.len())}
    }

    pub fn shuffle(&mut self) {
        /// Use Fisher-Yates shuffle algorithm
        let n = self.cards.len();
        for i in 0..(n - 2) {
            let j = rand::thread_rng().gen_range(i..n-1);
            self.cards.swap(i, j);
        }
    }

    pub fn reset(&self) {
        self.dealt.clear();
    }

    pub fn get_hand(&self, num_cards: u8) -> Vec<Card> {
        vec![]
    }
}



